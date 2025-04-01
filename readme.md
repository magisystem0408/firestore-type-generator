# fitype

CloudFireStore Type File Generator Tool.

## Generate Command

```
firegen -i ./example.yaml -o ./gen -t (ts | py) 
```

### Example Input YAML Schema

```yaml
- name: knowledgeGraphInfo # Variable name 
  export: true # Type Export True or False
  types: # Type Firebase Field Type
    id: string
    name: string
    author: string
```


### HomeBrew Installation

```bash
brew tap magisystem0408/firestore-type-generator
brew install fitype
```