import json
res="{\"squadName\":\"Super hero squad\",\"secretBase\":\"Super tower\",\"members\":[{\"secretIdentity\":\"Jane Wilson\",\"powers\":[\"Superhuman reflexes\",\"Damage resistance\",\"Million tonne punch\",\"Radiation blast\",\"Turning tiny\",\"Radiation resistance\"]},{\"secretIdentity\":\"Dan Jukes\",\"name\":\"Molecule Man\",\"age\":\"29\",\"name\":\"Madame Uppercut\",\"age\":\"39\"}],\"homeTown\":\"Metro City\",\"formed\":\"2016\",\"active\":\"true\"}"


a=json.loads(res)
print (a)