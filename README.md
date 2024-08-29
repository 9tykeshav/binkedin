# binkedin 
Binked is an attempt to clone Linkedin. only backend is planned as of now, frontend might be an option once the backend is completed or at the stable state.

# Running locally 
- Have postgreSQL installed
- Rust installed
- dotenv file with the following variables: 
```
DATABASE_URL = <database_url>
BINKEDIN_MEDIA = <path/to/folder_for_media>
```

- Create all neccesary tables from [Schemas](https://github.com/9tykeshav/binkedin/blob/master/schemas/schemas.sql)
- `Cargo run`, now you might have something on port 3000 if you are lucky enough.

# what now?
The closest thing to documentation for the API is the source code itself.
There's [this](api-docs) but its not really elaborative as of now 
