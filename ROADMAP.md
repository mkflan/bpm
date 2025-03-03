Databases:
- a group of passwords
- stored in an encrypted, password-protected text file

- design database file format (.bpmdb)


- Implement basic commands
- Add encryption
- Add authentication
- Switch to scroll TUI interface

- flow:
  - use create command to create .bpmdb file
    - creates a file with the proper format
    - no DB object created in rust code
  - open command to open .bpmdb file
    - checks password and decrypts
    - deserializes user provided file
  - do things on opened database with other commands
  - close command to close .bpmdb file
    - serialize updates data back to file
    - reencrypt
