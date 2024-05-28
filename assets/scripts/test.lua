Comp = nil

function dump(o)
   if type(o) == 'table' then
      local s = '{ '
      for k,v in pairs(o) do
         if type(k) ~= 'number' then k = '"'..k..'"' end
         s = s .. '['..k..'] = ' .. dump(v) .. ','
      end
      return s .. '} '
   else
      return tostring(o)
   end
end


function on_fixed_client(entity)
	if Comp == nil then
		Comp = world:get_type_by_name("Transform")
	end

	local playerId = world:get_component(entity, Comp)

	print(dump(playerId))
end
