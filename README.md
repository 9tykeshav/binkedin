# binkedin 
Binked is an attempt to clone Linkedin, made with axum & nextjs 
# Running locally 
- Have postgreSQL installed
- Rust installed
- dotenv file with the following variables: 
```
DATABASE_URL = <database_url>
BINKEDIN_MEDIA = <path/to/folder_for_media>

```
- dotenv for the frontend to be aware where to request the backend. Create it in /binkedin-web:
```
NEXT_PUBLIC_IP_ADDR_FOR_SERVICES= <where ever the backend is binded to>
```


- Create all neccesary tables from [Schemas](https://github.com/9tykeshav/binkedin/blob/master/schemas/schemas.sql)
- `cd binkedin` &  `cargo run`, now you might have something on port 3000 if you are lucky enough.
- `cd binkedin-web` & `npm run dev` and the frontend might also be available if youre lucky enough.

# what now?
The closest thing to documentation for the API is the source code itself.
There's [this](api-docs) but its not really elaborative as of now 
