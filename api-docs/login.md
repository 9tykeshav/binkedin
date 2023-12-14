GET /api/login HTTP/1.1
Content-Length: 42
Content-Type: application/json
Host: localhost:3000
User-Agent: HTTPie

{"email" : "keshav", "password": "122345"}

# login 
- returns 404 if doesnts exits 
- refer to does exists field 




# register
- SAME AS LOGIN ;
- returns internal server error for duplicate entry and 201 for sucess 
- refer to info field returned  



# post
- we have a blunder