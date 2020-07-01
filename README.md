# Gruit

[gruit](https://en.wikipedia.org/wiki/Gruit) is the ancestor of beer.

This is a project I've used to get acquainted with rust.
It's a simple API which takes beer recipe data, and returns some easy calculations which can be made from it, such as the color and bitterness.

## Setup

You need rust installed to run the app.
Once that is done, you can run `cargo test` and `cargo run` to run the tests and start the app.

## Usage

There are two endpoints. `POST /malt` and `POST /hop`.

All examples here are using the recipe from this
[Popcorn and Espelette Pepper Belgian Ale](https://beersmithrecipes.com/viewrecipe/3058169/popcorn-belgian-ale),
with the approximate efficiency from my home tool, the
[Klarstein Mundschenk](https://www.klarstein.fr/Electromenager/Distributeurs-de-boissons/Fabriquer-sa-propre-biere/Kit-de-brassage-de-biere-maison-cuve-a-maiche-2500W-inox.html)

### POST /malt

This endpoint takes a list of malts used in the beer recipe, and returns the theoretical color of the finished product, in EBC and in RGB formats.

```
curl -X POST http://localhost:8000/malt \
  -d '{
    "efficiency":72,
    "quantity":20,
    "malts":[
      {"quantity":3700,"ebc":8},
      {"quantity":800,"ebc":17.7},
      {"quantity":500,"ebc":45},
      {"quantity":250,"ebc":12},
      {"quantity":100,"ebc":5.9}]
  }'
```

### POST /hop

This endpoint takes a list of hops used in the beer recipe, and returns the theoretical bitterness of the finished product.

```
curl -X POST http://localhost:8000/hop \
  -d '{
    "quantity":20,
    "original_gravity":1056,
    "hops":[
      {"quantity":12,"alpha":12,"duration":60},
      {"quantity":10,"alpha":5.5,"duration":20},
      {"quantity":10,"alpha":5,"duration":20}
    ]
  }'
```
