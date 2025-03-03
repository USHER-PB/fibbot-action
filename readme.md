   # FIBBOT DOCUMENTATION

## INTRODUCTION

This project is based on calculating the fibonnaci of a given set of values and extracting their result so as to be post as comment when a pull request is been made.


## USAGE
Firstly the fibonnaci of a given set of values are calculated and their result are place as comment when a pr is been made.

- So for this i have use a workflow that will be trigger each time my pr is been made 
- A dockerfile so as to build and run my image 

- If you want to take a look on the repository  start by doing  

      git clone https://github.com/USHER-PB/fibbot-aciton.git
   
   to have  the actual repository locally on your machine 

-  run the program using 

              cargo run     

- then build your dockerfile and run your docker image by running 

      docker build -t fibbot .
  to build your docker image and finally 

      docker run -it --rm --name<container name > fibbot
  to run you docker image 

  ### Contribution

  if you are interested on bringing some small changes to the repository i invite you to look at this 

       git pull https://github.com/USHER-PB/fibbot-aciton.git



and finaly make a pull request so as for your contibution to be admit and merge to the main branch.
    


