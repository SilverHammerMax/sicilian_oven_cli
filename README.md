# Italy: The Game

Welcome to Italy: The Game. This game is an extension of the traveling salesman's problem: you will be given a set numer of cities, and you will have to find the quickest path through them. You
can play against your friends or preset challenges, as well as just playing against yourself. This game takes place in the South of Italy, and a map is attached to guide you around.

In this guide, we will cover the following 6 chapters:

1. [Basic Overview](#basic-overview)
2. [The Map](#the-map)
3. [Basic Gameplay](#basic-gameplay)
4. [Car Building](#car-building)
5. [Repairing, Refuelling, and Road Types](#repairing-refuelling-and-road-types)
6. [Player vs. Player](#player-vs-player)

## Basic Overview

The goal of the game is to travel to all the cities indicated by the game mode you're playing as quickly as possible. The smaller the time, the better. There are multiple different [game-provided cars](#generic-cars)
you can use, and you can also build your own car. Each car has a set of [attributes](#car-building) which will determine its speed, fuel usage, and reliability. There are also different [kinds of roads](#road-types) where certain cars
will perform better. Essentially, the entire game is an exercise in managing the fuel and condition of your car while plotting the fastest route between a given set of cities.

The ["challenge" mode](#challenges) gives you a set amount of cities and a target time to hit. Each challenge might have different conditions: some might ask you to start or end here, some might not ask you to start or
end anywhere in specific. The "random cities" mode is a [Player vs. Player mode](#player-vs-player). It's essentially a challenge mode that is randomized every time, where you can play against your friends. You can select
the amount of cities you want to go to and the seed for these cities. A seed is essentially a word or a number which will generate the same random cities across all games on all devices.

The final important "mode" is the Car Builder. Here, you can [build your own custom car](#car-building). You select the [engine](#engine), [gearbox](#gearbox), [chassis](#chassis), and [tires](#tires) that this car will use. Each part has a tradeoff between two
attributes, making each choice important. Every part has five different choices, each manufactured by the same five companies: Stellare, Veloce, Ardente, Solare, and Fiorente. Stellare cars will typically
be on the lower and safer end of the spectrum while Fiorente cars will typically be the biggest and riskier choices. You can control attributes of the car which will, in turn, control the speed of the car.

In terms of the map, there are five different kinds of [roads](#road-types): Highway, Asphalt, Stone, Unpaved, and Ferry. Ferries take the same time for all cars. Meanwhile, certain cars might go faster than others on
different roads, especially due to their [tires](#tires). You should plot out your path with your car's abilities on certain roads in mind. On the map, major cities are also designated. These are the only cities
where you can refuel or repair your car. This takes significant time, so you should plan to try and do this as sparingly as possible. You can only refuel or repair your car to the maximum.

Overall, the objective of this game is to go as fast as possible. Whether that's with friends or by yourself, speed is the name of the game. Good luck!

## The Map

Before discussing anything else, it's important to discuss the map:

![map](assets/mapofitaly.png)

The map covers 8 regions of Italy, with cities and routes placed across the entirety of the map. There are two very important distinctions to make with the map when you're using it:

1. The distinction between different kinds of [roads](#repairing-refuelling-and-road-types). There are five kinds of roads: highway, asphalt, stone, unpaved, and ferry. Each of these roads will result in a different travel time for every car, except
   the ferry; a ferry connection will always take the exact same amount of time regardless of the car. Certain cars might go quicker than others on highway and asphalt roads, while the reverse might be
   true on stone and unpaved roads. The way the car was built has a major effect on a car's performance on a given road, but we'll get into that later. Use the legend to differentiate between the different roads.
2. The distinction between a major and a minor city. A major city, as marked in the legend, is a location where you can [repair or refuel](#repairing-refuelling-and-road-types) your car (the mechanics of which will be explained later) and are frequently
   reference points for starting or ending a possible challenge (which will also be explained later). A major city is noticeably larger than a minor city, and always has a bright red outline. Minor cities might
   have different outlines depending on what roads connect to them. The legend should clearly mark all the possible options for how a city will look.

The legend, on the left-hand side of the map, notes all the different kinds of roads and cities as well as listing the name of each of the 8 regions. It's important to note that distance in-game might
not exactly correlate to the distance on the map. It's also important to note that visible geographical features (such as mountains or mountain ranges) have no bearing on any time calculations. However,
there is an increased amount of stone and unpaved roads in the more mountainous areas.

## Basic Gameplay

Welcome to Italy: The Game! You can use the arrow keys to navigate this and all other menus.
Some might ask you to select an input with your Y or N keys to signify yes or no respectively. The menu system itself should be fairly intuitive.
Once you start the game, you should be presented with three basic options:

> 1. [Challenges](#challenges)
> 2. [Random Cities](#player-vs-player)
> 3. [Build Car](#car-building)

We'll cover all of these options in the further chapters, but for now let's assume you want to drive around, free of expectations of requirements. Select "1. Would you like to start a challenge?" and
select the "Free Play" mode. There, you can select a car and a major city to begin from. If you've not already built a car, you can simply select a pre-built one. This mode will enable you to drive around.
Below is an example of a dialogue option.

> Welcome to Ragusa!
>
> Your current time is 0 hour(s) and 0 minute(s)!
>
> Your fuel is 26.
>
> Your path has been: ["Ragusa"]
>
> Your current list of missing cities is: []
>
> Where would you like to go?:
>
>  - Go to Marina di Ragusa, 33 km
>  - Go to Modica, 11 km
>  - Go to Comiso, 8 km
>  - Go to Giarratana, 17 km
>  - Submit your challenge or return to main menu
>  - Refuel
>  - Repair your car

The menu has 5 main parts. The first part welcomes you to whatever city you are in. The second part tells you how long you've spent driving. The third part tells you how many liters of fuel you have
left in your car. For every turn you take, your fuel will drop by a fixed amount proportional to how large the engine you selected is. For example, if you select a 1.6 Liter engine, you will lose 1.6
liters of fuel. So your fuel would go from 26 to 24.4. If the amount of fuel you have in your car drops below 0, you must restart.

The fourth part of the menu lists your path up until that point. It will include all the cities you've travelled to and any [refuelling/repair stops](#repairing-refuelling-and-road-types) you may have made. The fifth part of the menu tells
you your list of missing cities. During [challenges](#challenges) or [Player vs. Player](#player-vs-player), you'll have a set amount of cities you have to travel to, and the "missing cities" list will make sure you stay informed of what
cities you need to travel to.

Finally, the sixth part of the menu offers you options. You must select one to continue. You will have, at most, four different kinds of options:

1. An option to travel to another city. This option will list the city and the distance to that city. You can use the map to check where that city is, whether you would like to go to that city, and what
   kind of road connects the city you're currently at and your target city.
2. An option to "Submit your challenge or return to main menu". By selecting this option, you are either submitting your [challenge](#challenges) or returning to the main menu. Make absolutely sure your challenge is complete
   before clicking this button: there is no way to go back.
3. An option to refuel. This option will only be available at major cities, and it will allow you to [refuel your car](#refuelling).
4. An option to repair your car. This option will also only be available at major cities, and it will allow you to [repair your car](#repairing).

There may also occasionally be a part of the menu that gives you a warning for low fuel. This warning will intuitively only appear when you are low on fuel. There may also be a text that tells you that
your challenge is complete, although this would only come up during challenge mode. This is your signal to select the "submit your challenge" dialogue option.

When playing the game, remember that the goal is to go as fast as possible. This means trying to minimize time spent refuelling and going between cities. Remember that you lose fuel at a set amount per
turn and that distance does NOT impact fuel usage. It might seem counter-intuitive, but traveling 20 kilometers from city A to city B while going through city C uses more fuel than traveling 30 kilometers
going from city A to city B. The actual time spent between cities corresponds to a complicated formula that's expanded on in chapter 5. The same applies to the car's condition, which will also be expanded
on in chapter 5.

### Challenges

Challenges are one of the most essential parts of the game. There are many challenges in the game. Each challenge has a predetermined list of cities that you must pass through,
and most have a city that you must begin and/or end at. There are pre-set times for all challenges to give a benchmark of what you're competing against. Instead of being a race against
a friend, it's more a race against the computer.

Some challenges may not have a set city to start at, in which case you'll be prompted to select a major city to begin at.

## Car Building

There are 4 parts to every car, and each part has an effect on how the car performs: the engine, the chassis, the gearbox, and the tires. Car Speed is impacted by four variables: Weight,
Horsepower, Condition, and Tire Grip. Each part has an effect on at least one of these variables, while some also have an effect of fuel storage or usage. The following section details
what all the parts do and what gain or loss to a given variable they might offer. To see how the car's speed using these variables is calculated, please refer to the [Road Types](#road-types)
section.

It's important to note that every part has five options, with each option being provided by a different in-universe company. For example, the Stellare company will always provide the smallest engine, smallest chassis,
weakest (but most reliable!) gearbox, and tires that significantly prefer unpaved roads than paved.

### Engine

The engine is, naturally, an important part of the car. The main tradeoff in picking an engine is between fuel mileage and horsepower. The amount of liters next to the engine name is its capacity,
and therefore how much fuel it will use per turn. A 5.2L engine will use 5.2 liters of fuel per turn while a 2.4L engine will only use 2.4 liters. Meanwhile, the horsepower is a big deciding
factor in how fast a car can go. Each engine also has a weight figure, but the Horsepower-to-Weight ratio continually increases the stronger the engine gets. The following are the engine options:

| Manufacturer | Name | Engine Size | Horsepower | Weight |
| :---: | :---: |:-----------:|:----------:|:------:|
| Stellare | Rigatoni |    2.4L     |   142HP    | 235KG  |
| Veloce | Penne |    3.2L     |   200HP    | 357KG  |
| Ardente | Bowtie |    3.6L     |   220HP    | 372KG |
| Solare | Bucatini |    4.6L     |   276HP    | 406KG |
| Fiorente | Lasagna |    5.2L     |   320HP    | 432KG |

### Gearbox

The gearbox is the most important part of the car when it comes to the condition and reliability. The main tradeoff in picking a gearbox is between reliability and horsepower. There are
gearboxes who will result in very large horsepower gains, but will also result in much lower reliability. For example, one kind of gearbox will add 220HP to your car, but will take away 3.5%
of your condition every turn. The following are the gearbox options:


| Manufacturer | Name | Horsepower Boost | Reliability |
| :---: | :---: |:----------------:|:------------:|
| Stellare | Provolone | +0HP | -1% |
| Veloce | Mozzarella | +40HP | -1.5% |
| Ardente | Gorgonzola | +80HP | -2% |
| Solare | Cheddar | +120HP | -2.5% |
| Fiorente | Parmesan | +220HP | -3.5% |

### Chassis

The chassis is the biggest factor in determining the weight of your car. The main tradeoff in picking a chassis is between weight and fuel storage. The bigger the chassis, the more it weighs,
but the more fuel you can store. The following are the chassis options:

| Manufacturer |   Name    | Weight | Fuel Tank |
| :---: |:---------:|:------:|:---------:|
| Stellare | Ciabatta  | 705KG  |   33.6L   |
| Veloce | Panettone | 778KG  |    37L    |
| Ardente |  Rosetta  | 861KG  |    41L    |
| Solare | Focaccia  | 914KG  |   43.5L   |
| Fiorente |  Filone   | 996KG  |   47.4L   |

### Tires

The tires are the part which will give you life or give you death on a given surface. The main tradeoff in picking a tire is between your performance on paved and unpaved roads. The
more GGC (Gravel Grip Coefficient) you have, the better you'll perform on unpaved roads. The more AGC (Asphalt Grip Coefficient) you have, the better you'll perform on paved roads. The
following are the chassis options:

| Manufacturer | Name | AGC | GGC |
| :---: | :---: |:---:|:---:|
| Stellare | Fox | 1 | 10 |
| Veloce | Boar | 4 | 8 |
| Ardente | Wolf | 6 | 6 |
| Solare | Horse | 8 | 4 |
| Fiorente | Roadrunner | 10 | 1 |

### Generic Cars

There are 4 generic cars which are constructed using in-game parts. For simplicity, I've listed all the important attributes for all the generic cars below:

|    Car Name    | AGC | GGC | Horsepower | Fuel Usage | Fuel Tank | Weight | Reliability |
|:--------------:| :---: | :---: |:----------:|:----------:|:---------:| :---: | :---: |
| Il Commandante | 8 | 4 |    222     |    2.4     |   37.0    | 1013 | -2% |
| Il Grande | 6 | 6 |    440     |    5.2     |   47.4    | 1328 | -2.5% |
| Il Capo | 4 | 8 |    240     |    3.2     | 33.6 | 1062 | -1.5% |
| Il Generalissimo | 8 | 4 |    220     |    3.6     | 37.0 | 1150 | -1% |

## Repairing, Refuelling, and Road Types

### Repairing

Condition is a critical factor of each car. It has a significant impact on car speed and is important to keep high. However, there is a balance towards
when you should repair. Due to a large fixed-time cost of repairing the car, it's impractical to repair at every possible opportunity. Simultaneously, you can
only repair your car at major cities, which means that you have to plan your repair stops far in advance. You can also only repair to the maximum 100% condition,
so "partial" repairs are not allowed. The formula for repairing the car is the following:

> 145 - Condition

This formula's output should be considered in terms of minutes. For example, let's say your car's condition is at 70% and you would like to repair it. The time
it would take to be repaired is:

> 145 - 70
>
> 75 minutes

Effectively, this guarantees a minimum of 45 minutes on a repair stop along with 1 minute for every percent under 100 your condition is.

### Refuelling

Refuelling is very similar to repairing in that it has a fixed cost to encourage less fuel stops. You similarly do not have the option to only partially refuel
your car and must always refuel it to the full fuel tank. However, the cost of refuelling is significantly less than that of repairing. The fixed cost is much
lower and a fuel stop will typically take you less than a repair stop.

> 10 + (1.5 * missing_fuel)

This formula's output should be considered in terms of minutes. For example, let's say your car's fuel level is at 30 Liters and your fuel tank can fit 50 Liters.
The time it would take to be refuelled is:

> 10 + (1.5 * (50 - 30))
> 10 + (1.5 * 20)
> 10 + 30
> 40 minutes

Effectively, this guarantees a minimum of 10 minutes on a refuel stop along with 1.5 minutes for every liter of fuel missing from the tank.

### Road Types

There are five different kinds of roads. Highways, Asphalt, Stone, Unpaved, and Ferries. Ferries are the only type of "road" where your car has no
bearing on the overall travel time. It will be the same for all cars.

There will be 4 stats considered to calculate the speed of a given car on a given surface. Weight, Horsepower, Condition, and either AGC or GGC depending
on the road type. Highways and Asphalt roads use AGC and Stone and Unpaved roads use GGC. The following formulas are used to calculate the speeds of all
the cars in terms of kilometers per hour. Each formula will be applied when on the given road type

> Highway:
>
> 2 * (Horsepower * Condition * AGC) / Weight

> Asphalt:
>
> (Horsepower * Condition * AGC) / Weight

> Stone:
>
> (3/4) * (Horsepower * Condition * GGC) / Weight

> Unpaved:
>
> (1/2) * (Horsepower * Condition * GGC) / Weight

Each of these formulas will be used in the implied conditions to calculate whatever the speed is. Let's say, for example, that your car
has 300 Horsepower, 80% Condition, 6 AGC, weighs 1000 kilograms, and is traveling on an Asphalt road. Your speed would be:

> (300 * 80 * 6) / 1000
>
> 144000 / 1000
>
> 144 kilometers per hour

Assuming that the road you're traveling on is 20 kilometers long, the amount of time to drive the road would be:

> 20 km / 144 kph
>
> ~0.1388 hours
>
> ~8.3 minutes!

Ferries are the only example where none of these factors matter. The ferry formula is:

> Ferry:
>
> 15 minutes + (2.5 * distance)

This formula provides a 15-minute onboarding/offboarding time while the ferry takes 2.5 minutes to travel every kilometer.

## Player vs. Player

In Player vs. Player (PvP) mode, two people can play against each other. If you select "random mode" when beginning the game, this allows you to
play against a friend. Each person will be prompted two questions, and should respond the same thing to each question:

1. How many cities would you like to go to?
2. What seed would you like to use (Leave blank for a random seed)

The first option will give a certain amount of cities that both you and your opponent have to reach, while the second option will ensure that both
you and your opponent are given the same randomly selected cities (provided you submit the same seed). A seed can be anything: a word, a number, a phrase.
As long as it is identical between both players, they will receive identical cities. Each player can then select either a generic car or a custom-built
car and begin racing. The winner is judged by whoever sets the faster time.

Unless if both players set a rule against it, you can both start and end at any city. When you begin, you will be given a list of all the major cities
to cycle through and select the city you think will help you the most.

It is up to the players to select the conditions of the battle. For example: how many attempts they get at the challenge, how much time is allowed
to complete the attempt(s), and whether you can use different cars.