# [Mongo Crud Operations](https://www.mongodb.com/docs/manual/crud/)

## Create Operations  

If the collection does not exist, insert operation will create new `collection`

```py
>>>db.collection.insertOne()
>>>db.collection.insertMany()
```

Insert operation targets the single collection  
All write operations are `atomic`  

## Read Operations  

> db.collection.find()

### Query Documents  

Select all documents

>db.collection.find({})

```sql
SELECT * from table
```

#### Equality Condition

~~~
{ <field> : <value> }
~~~

```sql
>>> db.inventory.find({"status":"D"})

SELECT * FROM inventory where status = "D";

```

#### In Condition 

~~~
{<field>: {<operator1>: <value1>}, ...}
~~~

```sql
db.inventory.find({'status':{'$in': ["A","D"]}})

SELECT * FROM inventory where status in ('A','D');

```

#### AND Condition

```sql
db.inventory.find({"status":"A","qty":{"$lt":30}})

SELECT * from inventory where status = 'A' and qty < 30; 
```

#### OR Condition

`$or` operator joins each clause with logical `OR`  

```sql
db.inventory.find({"$or":[{"status":"A"},{"qty":{"$lt":30}}]})
SELECT * from inventory where status = 'A' OR qty < 30;
```

#### AND as well as OR

```sql
db.inventory.find(
    {"status": "A", "$or": [{"qty": {"$lt": 30}}, {"item": {"$regex": "^p"}}]}
)

SELECT * FROM inventory WHERE status = "A" and ( qty < 30 OR item LIKE "p*")

```


### Query On embedded/Nested Document

```json
{
            "item": "journal",
            "qty": 25,
            "size": SON([("h", 14), ("w", 21), ("uom", "cm")]),
            "status": "A",
}
```

#### Matching 

To specify equality condition on a embedded struct fied use `query filter` 

```text
{<field>: <value>}
db.inventory.find({"size": SON([("h", 14), ("w", 21), ("uom", "cm")])})
```

Value is the document to match  
Equality match on whole embedded document require an exact match of the `Value` document including the field order  

#### Equality Match on nested field  

Use `.` operator to query the nested document 

```python
db.inventory.find({"size.uom": "in"})
db.inventory.find({"size.h": {"$lt": 15}})
# 
db.inventory.find({"size.h": {"$lt": 15}, "size.uom": "in", "status": "D"})
```

## Query An Array

```python
db.inventory.insertMany([
   { item: "journal", qty: 25, tags: ["blank", "red"], dim_cm: [ 14, 21 ] },
   { item: "notebook", qty: 50, tags: ["red", "blank"], dim_cm: [ 14, 21 ] },
   { item: "paper", qty: 100, tags: ["red", "blank", "plain"], dim_cm: [ 14, 21 ] },
   { item: "planner", qty: 75, tags: ["blank", "red"], dim_cm: [ 22.85, 30 ] },
   { item: "postcard", qty: 45, tags: ["blue"], dim_cm: [ 10, 15.25 ] }
]);
```

### Match an array

```python
db.inventory.find( { tags: ["red", "blank"] } )
```

`$all`

Find an array that contains both the elements "red" and "blank" without regards to order 

>db.inventory.find( { tags: { $all: ["red", "blank"] } } )



## Update Documents

Updates has following options

```python
db.collection.updateOne(<filter>, <update>, <options>)
db.collection.updateMany(<filter>, <update>, <options>)
db.collection.replaceOne(<filter>, <update>, <options>)
```

### In Collections

`$set` operator to modify the field values 

```text
{
  <update operator>: { <field1>: <value1>, ... },
  <update operator>: { <field2>: <value2>, ... },
  ...
}
```

`$set` will create the field if the fied does not exist 

```js
db.inventory.updateOne(
   { item: "paper" }, // WHERE condition
   {
    
     $set: { "size.uom": "cm", status: "P" }, // SET clause
     $currentDate: { lastModified: true }
   }
)
db.inventory.replaceOne(
   { item: "paper" }, // WHERE
   { item: "paper", instock: [ { warehouse: "A", qty: 60 }, { warehouse: "B", qty: 40 } ] }
)

```

### Atomicity 

Atomic on the level of single document

## Delete Documents

```js
db.collection.deleteMany()
db.collection.deleteOne()
```

### Delete all documents

>db.collection.deleteMany({})

### Delete all documents that matches the condition

Pass `filter` paramenters to the `deleteMany` function
>db.inventory.deleteMany({ status : "A" })


### Delete Behaviour

1. Donot drop `index` even if deleting all the documents from a collection

2. 