# search_json
This crate is to process deep neted json struct. The create expose two simple api:

##  flat_json(js:&str)->Vec<String>: convert arbitrary deep nested json to flatten row.
From
    ```
    {
  "squadName": "Super hero squad",
  "homeTown": "Metro City",
  "members": [
    {
      "name": "Molecule Man",
      "age": 29,
      "secretIdentity": "Dan Jukes",
      "powers": [
        "Turning tiny",
        "Radiation blast"
      ]
    },
    {
      "name": "Madame Uppercut",
      "age": 39,
      "secretIdentity": "Jane Wilson",
      "powers": [
        "Million tonne punch",
        "Superhuman reflexes"
      ]
    }
  ]
}
    ```
   To
   
    
    ```
data.clone() = [
    "ROOT{}--->homeTown:::Metro City",
    "ROOT{}--->members:::[]--->0>>>{}--->age:::29",
    "ROOT{}--->members:::[]--->0>>>{}--->name:::Molecule Man",
    "ROOT{}--->members:::[]--->0>>>{}--->powers:::[]--->0>>>Turning tiny",
    "ROOT{}--->members:::[]--->0>>>{}--->powers:::[]--->1>>>Radiation blast",
    "ROOT{}--->members:::[]--->0>>>{}--->secretIdentity:::Dan Jukes",
    "ROOT{}--->members:::[]--->1>>>{}--->age:::39",
    "ROOT{}--->members:::[]--->1>>>{}--->name:::Madame Uppercut",
    "ROOT{}--->members:::[]--->1>>>{}--->powers:::[]--->0>>>Million tonne punch",
    "ROOT{}--->members:::[]--->1>>>{}--->powers:::[]--->1>>>Superhuman reflexes",
    "ROOT{}--->members:::[]--->1>>>{}--->secretIdentity:::Jane Wilson",
    "ROOT{}--->squadName:::Super hero squad",]
    ```
  ##  nest_json(v:&Vec<String>)->String: reverse above process.
  
  # By this way,users can process deep nested with string tools like regular expression etc.,and trun string back to arbitrary nested json.  
