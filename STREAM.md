# DATE : 30th June 2026

## So Ig todays goal is to

1. take user input for images (if user wants to generate images)
2. take user input for videos (if user wants to generate videos)

about `flags `, I am thinking of 2 ways we can do it

1. keep same flag for input and detect whether user wants to generate img or video based on source extension

**PROS**

- keeps flag simple , ux is better

**CONS**

- devs exp gets worse as we'll have to do matching part

2. keep different flag for input such as -i for image and -v for video ?

**PROS**

- devs can just check flag instead of matching extension

**CONS**

- different flag , so users should know that but ig they aren't that dumb , are they ?

ig i'll go with differnt flag

Ok so now thinking about flag, ig i'll go with -img for image and -video for video

So flag part is done , next part is setting correct font/dimension depending on img and video because as you might have noticed, ascii art was weird
Anyways that's it for today, will continue from here on some other day . BYE :)
