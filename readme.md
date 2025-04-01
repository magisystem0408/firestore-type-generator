## CloudFireStore Type Generator 

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