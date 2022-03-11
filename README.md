    _____ _        _   _             __  __           _            
   / ____| |      | | (_)           |  \/  |         | |           
  | (___ | |_ __ _| |_ _  ___  _ __ | \  / | __ _ ___| |_ ___ _ __ 
   \___ \| __/ _` | __| |/ _ \| '_ \| |\/| |/ _` / __| __/ _ \ '__|
   ____) | || (_| | |_| | (_) | | | | |  | | (_| \__ \ ||  __/ |   
  |_____/ \__\__,_|\__|_|\___/|_| |_|_|  |_|\__,_|___/\__\___|_|    
#### Reverse Shell Session Manager

### project objective 
Station master is a presitance tool in which establish and manages reverse shells. The best part is that it is fully in rust! 

### Visual
+----------+  +----------+  +----------+
|Computer 1|  |Computer 2|  |Computer 3|
+----+-----+  +----+-----+  +-----+----+
     |             |              |
     |             |              |
     +-----+       |        +-----+
           |       |        |
           |  +----+-----+  |
           |  |Station   |  |
           +--+   Master +--+
              |          |
 +----------+ +---------++
 |>_        |           |
 |          +-----------+
 |          |
 |          |
 +----------+


### Stages
#### Stage 1
- [x] Make a CLI interface with meta Command
- [x] Session Objects?
  - [x] netcat session
  - [] ssh session

#### Stage 2
- [] Handle multiple include on selected ports for one session type
- [] More advance session objects like rot-13 and xor
- [] ?Web UI?
