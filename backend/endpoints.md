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
#### GET
Requires:
- JSON like this:
```json
{
  username: ?,
  password_hash: ?
}
```

Returns:
- `200 OK` with a valid auth token in the response AUTHORIZATION header
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
  username: ?,
  password_hash: ?,
  email: ? (**OPTIONAL**)
  phone: ? (**OPTIONAL**)
}
```
> *_At least one of the email and phone fields should be filled_*

Returns:
- `201 CREATED`
- `409 CONFLICT` - username already exists
- `400 BAD REQUEST` - missing fields or malformed phone or email
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
  username: ?,
  email: ? (OPTIONAL),
  phone: ? (OPTIONAL),
  bio: ? (OPTIONAL),
  friends: [array of ids],
  level: {
    level: ?,
    xp: ?
  },
  progress: {
    course: ?,
    unit: ?,
    sector: ?,
    level: ?,
    task: ?
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
- `200 OK`
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
---

## Answer
`/answer`
### Methods
#### POST
Requires:
- valid auth token in AUTHORIZATION header
- JSONized Answer struct

Returns:
- `201 CREATED`
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

