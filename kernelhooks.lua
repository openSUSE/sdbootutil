kernelhooks = { debug = nil } -- package

kernelhooks.found = {}
kernelhooks.legacy = {} -- entries in /boot

kernelhooks.imagename = "vmlinuz" -- FIXME: arch dependent!

kernelhooks.kernelpattern = string.format("^/usr/lib/modules/(.*)/%s", kernelhooks.imagename)
-- beware of crazy lua patterns, - is similar to * and % is escape character
kernelhooks.legacypattern = string.format("^/boot/%s%%-(.+)", kernelhooks.imagename)

-- for testing outside rpm
if posix == nil then
	posix = require("posix")
end

function _log(msg)
	if kernelhooks.debug then
		print(kernelhooks.debug .. ": " .. msg)
	end
end

function rootpath(path)
	_, _, p = string.find(path, "^/usr(/.*)")
	return p
end

function dirname(path)
	_, _, d, b = string.find(path, "^(.*)/([^/]*)")
	return d
end

function kernelhooks.filter(path)
	_, _, kver = string.find(path, kernelhooks.kernelpattern)
	if kver then
		_log("found kernel " .. kver)
		kernelhooks.found[kver] = path
		return
	end
	_, _, kver = string.find(path, kernelhooks.legacypattern)
	if kver then
		_log("kernel " .. kver .. " in /boot")
		kernelhooks.legacy[kver] = 1
		return
	end
end

function kernelhooks.add()
	for kver in pairs(kernelhooks.found) do
		if kernelhooks.legacy[kver] then
			_log("not adding " .. kver .. " due to legacy /boot location")
		else
			_log("adding " .. kver)
			os.execute("/usr/bin/sdbootutil add-kernel " .. kver)
		end
	end
end

function kernelhooks.remove()
	for kver in pairs(kernelhooks.found) do
		if kernelhooks.legacy[kver] then
			_log("not removing " .. kver .. " due to legacy /boot location")
		else
			_log("removing " .. kver)
			os.execute("/usr/bin/sdbootutil remove-kernel " .. kver)
		end
	end
end

