# duolingo-for-coding

# Project set up
This is a detailed guide how I run this child of satan.

***Warning***: it is absolutely *not-secure*, *barely working* and written the worst way possible. Needless to say I most certainly ***do not recommend running this by yourself*** (but there you go).

**Environment:** Ubuntu 22.04

**Needed software:** NodeJS (npm), NextJS, Google Chrome (sic!), Rust compiler, MySQL service


### Follow step by step:

1. Run *MySQL* service, go to *{project_dir}/backend* and run command ```sudo mysql < all_databases.sql```;
2. Run the server - in the same directory run ```cargo run```;
	- *(optionally)* Instead run ```cargo run -- -l``` to print logs into console, which may be helpful if this project dies during the tests.
3. Run the frontend - go to *{project_dir}/frontend* and run ```npx next dev```
	- If you face any issues, rebuild everything with *npx* and *npm*.
	- Yes, it **must** run through *dev*, otherwise it doesn't work. 
4. Run Chrome **from command line**  like this: ```google-chrome --disable-web-security --user-data-dir="/tmp/chrome_dev"```
	- Please note: use this command *for this purpose only*. Do not open any other websites, it is a huge security risk for your device.
5. In Chrome go to *localhost:3000*, there is our application UwU
	- ***And do not try to open any other website, I warned you.***
	- *(optionally)* Open another tab, go to *localhost:8080/test* for test endpoint check.
	
Questions? I don't care, we deliver it by ***Monday, Sep. 9th***.
