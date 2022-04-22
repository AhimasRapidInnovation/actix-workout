# [Documents](https://www.mongodb.com/docs/manual/core/document/)

`Mongo` stores data as `bson` documents  
[`BSON`](https://bsonspec.org/) is binary representing of `JSON` documents

Document structure

```json
{
   field1: value1,
   field2: value2,
   field3: value3,
   ...
   fieldN: valueN
}
```

Values of the filed should be [bson data type](https://www.mongodb.com/docs/manual/reference/bson-types/)


## Field Name

String type  
_id is reserved for use as a primary key  

_id value should be Unique in the collection, and it is immutable  

Field name cannot contains `null` 

## Dot Notation  

To access the element of the array and to access the field of the embedded document 

### Arrays  

Zero based indexed position  

> "<array>.<index>"

```js
{
   ...
   contribs: [ "Turing machine", "Turing test", "Turingery" ],
   ...
}

```

To query the 3rd item  
>"contribs.2"

### Embedded Document  

>"(embedded document).(field)  

```{
   ...
   name: { first: "Alan", last: "Turing" },
   contact: { phone: { type: "cell", number: "111-222-3333" } },
   ...
}
>>> name.last
>>> contact.phone.type
```


## Documents Attributes

* maximum `bson` documents size  is `16mb`

* `BSON` documents are ordered

* Preserves the order of the document fied except (_id (always first)


## _id Field  

1. Act as a `primarykey`  

2. If `document` omits  `id` mongo automatically generate   ObjectID for that `_field`

3.   
