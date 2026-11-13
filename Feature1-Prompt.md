I want to implement all funtions from /js/rmathjs in /rust/rmathrs/src/functions

/js/rmathjs/ exports
export {
  bignum,
  fraction,
  real, decimal, bigdecimal,
  complex
} from "./dtype/index.ts";

this is for basic num operation

no i don't want to re implement the entire logic in rust, so instead use malachite to implement these all classes as wrapper on malachite type. 
dtype -> numbers

for functions start implementation for functions/elementary

and for rest all functions give me estimate how many functions can completed using malachite. i just want wrapper over malachite functions for now


First create a plan 