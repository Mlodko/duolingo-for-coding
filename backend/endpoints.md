# Structs

## Answer
```json
{
  "id": UUID,
  "user_id": UUID,
  "task_id": UUID,
  "content": AnswerContent[?]
}
```

## AnswerContent

### Multiple choice variant
```json
{
  "selected_answers": [String]
}
```

### Open question variant
```json
{
  "content": String
}
```

### Construct from parts variant

```json
{
  "parts": [String]
}
```

## Verification Result

```json
{
  "correct": bool,
  "explanation": String? (**Only for open question variant**)
}
```

## Task
```json
{
  "id": UUID,
  "title": String,
  "content": TaskContent,
  "tags": [Tag]
}
```

### Tag
```json
{
  "id": UUID,
  "name": String
}
```

## TaskContent

### Multiple Choice Variant
```json
{
  "question": String,
  "choices": [String]
}
```

### Open Question Variant
```json
{
  "content": String
}
```

### Construct from parts variant

> ***CURRENTLY UNIMPLEMENTED, BUT WILL SOON(TM)***

```json
{
  "parts": [String]
}
```

# Disclaimers

### Json optional values
If a value is denoted like this: 
```json
{
  "Key": Type[?]
}
```
it means that the value is **optional**.

# Current endpoints
---

## Test endpoint

`/test`

The simplest endpoint to test whether the server is running.

### Methods
#### GET
Returns:
- `200 OK`
---

## Authorization
Not an endpoint, but all endpoints that require the auth token can also return:
- `400 BAD REQUEST` - no token in request or malformed token
- `403 FORBIDDEN` - token expired
- `401 UNAUTHORIZED` - token is not in the database
- `500 INTERNAL SERVER ERROR`
---

## User
`/user`

This endpoint and it's subendpoints manage the User structs

### Methods
#### PUT
Requires:
- a valid auth token in the AUTHORIZATION header
- serialized User object in JSON

Returns:
- `200 OK`
- `500 INTERNAL SERVER ERROR`
---

`/user/login`
### Methods
#### POST
Requires:
- JSON like this:
```json
{
  "username": String,
  "password": String
}
```

Returns:
- `200 OK` with a valid auth token in the response AUTHORIZATION header with the user id as json
- `403 FORBIDDEN` - if the credentials are wrong
- `500 INTERNAL SERVER ERROR`
---

`/user/register`
### Methods
#### POST
Requires:
- JSON:
```json
{
  "username": String,
  "password": String,
  "email": String[?],
  "phone": String[?]
}
```
> *_At least one of the email and phone fields should be filled_*

Returns:
- `201 CREATED`
- `409 CONFLICT` - username already exists
- `400 BAD REQUEST` - missing fields or malformed phone or email
- `500 INTERNAL SERVER ERROR` 
---

`/user/logout`
Requires:
- valid auth token in AUTHORIZATION header

Returns:
- `200 OK`
- `500 INTERNAL SERVER ERROR`
---

`/user/{id}`
### Methods
#### GET
Requires:
- valid auth token in AUTHORIZATION header
- valid user id in the path (`{id}`)

Returns:
- `200 OK` with info on the selected user in JSON:
```json
{
  "username": String,
  "email": String[?],
  "phone": String[?],
  "bio": String[?],
  "friends": [String (UUID)],
  "level": {
    "level": uint32,
    "xp": uint32
  },
  "progress": {
    "course": uint32,
    "unit": uint32,
    "sector": uint32,
    "level": uint32,
    "task": uint32
  }
}
```
- `400 BAD REQUEST` - invalid id in path
- `500 INTERNAL SERVER ERROR`

#### DELETE
Requires:
- valid auth token in AUTHORIZATION header
- valid user id in the path (`{id}`)

Returns:
- `204 NO CONTENT`
- `400 BAD REQUEST` - invalid id in path
- `500 INTERNAL SERVER ERROR` 
---


## Task
`/task/{id}`
### Methods
#### GET
Requires:
- valid task id in the path (`{id}`)

Returns: 
- `200 OK` with JSONized Task struct
- `400 BAD REQUEST` - invalid id in path
- `404 NOT FOUND` - no task with specified id
- `500 INTERNAL SERVER ERROR`
---

`/task/random`
### Methods
#### GET
Requires: Nothing :))

Returns:
- `200 OK` with a randomly chosen task object in json
- `404 NOT FOUND` (shouldn't happen tho) OR `500 INTERNAL SERVER ERROR` 
---

`/task/next`
### Methods
#### POST
Requires: 
- a json array of already seen tasks's ids (they will be excluded from the rng)

Returns:
- `200 OK` with a randomly chosen task object in json
- `404 NOT FOUND` (shouldn't happen tho) OR `500 INTERNAL SERVER ERROR` 
---

## Answer
`/answer`
### Methods
#### POST
Requires:
- valid auth token in AUTHORIZATION header
- Answer form in json like this:
```json
{
  "user_id": Uuid,
  "task_id": Uuid,
  "content": AnswerContent[?]
}
```

Returns:
- `201 CREATED` with id in `Location` header and with a Verification Result struct in the body
- `500 INTERNAL SERVER ERROR`

#### PUT
Requires:
- valid auth token in AUTHORIZATION header
- JSONized Answer struct

Returns:
- `200 OK`
- `500 INTERNAL SERVER ERROR`

#### DELETE
Requires:
- valid auth token in AUTHORIZATION header
- JSONized Answer struct

Returns:
- `204 NO CONTENT`
- `500 INTERNAL SERVER ERROR`
---

`/answer/{id}`
### Methods
#### GET
Requires:
- valid answer id in path
- valid auth token in AUTHORIZATION header

Returns:
- `200 OK` with JSONized Answer struct
- `404 NOT FOUND`
- `400 BAD REQUEST`
- `500 INTERNAL SERVER ERROR`

