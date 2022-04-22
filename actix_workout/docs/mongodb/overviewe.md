# [Overview](https://www.mongodb.com/docs/manual/core/databases-and-collections/)

`MongoDb` stores records as documents(BSON Document)  
`Document` gathered together in `collections`  
A `Database` stores one or more `collections`  


## Collections

Collections is like table 

### Creating a collections

If collections does not exits `mongodb` creates `collections` when first storing the data  

```js
db.myNewCollection2.insertOne( { x: 1 } )
db.myNewCollection3.createIndex( { y: 1 } )
```

#### Explicit Creation

`db.createCollection()` create collection with various options like  

1. Maximum size of the document

2. Validation Rule

### Modifying Document Structure  

Update the `document` to the new structure  

### Unique Identifier

`Collections` are assigned an `immutable UUID`   
`UUID` remains  the same across all members of a replica set shared in a shared cluster  


To retrieve `UUID` for a `collection` run `listCollections` or `db.getCollectionInfo()` 


## Views 

View is q `queryable`  , `Mongo` does not persist the view content to disk  
view content is computed on demand when a client quries the view , `mongo` does not support `write` operation on `views`  

UseCase is for joining table information 