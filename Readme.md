# Z-OS Attempt 7

## Usage

- Build: ```cargo build```
- Run: ```cargo run```
- Test: ```scripts/test.sh```
- Setup git hooks: ```scripts/git_hooks.sh```
- Build docker: ```docker build .```
- Launch in docker (go to [http://localhost:8080/vnc.html](http://localhost:8080/vnc.html) to view and interact): ```docker-compose up```

## TODO
[From here](https://os.phil-opp.com/async-await/#possible-extensions):

- Task scheduling
- Task spawning
- Threads
- Load balancing tasks

Not from the guide:

- On screen clock
- Text UI (login screen, basic CLI)
