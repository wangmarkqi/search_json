import yaml
import json
file="D://data/txt/law/鞍山和盛商城发展有限公司.txt"
with open(file,"rb") as f:
    dic=json.load(f)
print (dic)


with open(r'test.yaml', 'w') as file:
    documents = yaml.dump(dic, file,allow_unicode=True)