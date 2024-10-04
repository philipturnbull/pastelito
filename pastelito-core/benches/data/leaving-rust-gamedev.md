# Leaving Rust gamedev after 3 years

* Once you get good at Rust all of these problems will go away
* Rust being great at big refactorings solves a largely self-inflicted issues with the borrow checker
* Indirection only solves some problems, and always at the cost of dev ergonomics
* ECS solves the wrong kind problem
* Generalized systems don't lead to fun gameplay
* Making a fun & interesting games is about rapid prototyping and iteration, Rust's values are everything but that
* Procedural macros are not even "we have reflection at home"
* Hot reloading is more important for iteration speed than people give it credit for
* Abstraction isn't a choice
* GUI situation in Rust is terrible
* Reactive UI is not the answer to making highly visual, unique and interactive game UI
* Orphan rule should be optional
* Compile times have improved, but not with proc macros
* Rust gamedev ecosystem lives on hype
* Global state is annoying/inconvenient for the wrong reasons, games are single threaded.
* Dynamic borrow checking causes unexpected crashes after refactorings
* Context objects aren't flexible enough
* Positives of Rust
* Closing thoughts

The article is also available in [Chinese](https://microblock.cc/blog/leaving-rust-gamedev).

*Disclaimer: This post is a very long collection of thoughts and problems I've had over the years, and also addresses some of the arguments I've been repeatedly told. This post expresses my opinion the has been formed over using Rust for gamedev for many thousands of hours over many years, and multiple finished games. This isn't meant to brag or indicate success, but rather just show there has been more than enough effort put into Rust, to dispel the the commonly said "once you gain enough experience it'll all make sense" argument.*

_This post isn't a scientific evaluation or an A/B study. It's my personal opinion after trying to make Rust gamedev work for us, a small indie developer (2 people), trying to make enough money to fund our development with it. We're not one of those developers that have infinite money from an investor and years to burn. If you're in that category and are happy to build systems for years, none of the below will apply. I'm looking at things from the perspective of "I want to make a game in 3-12 months maximum and release it so that people can play it and I can make some money from it.". This is not written from a perspective of "I want to learn Rust and gamedev seems fun", which even though is a valid goal, is not in any way aligned with what we want, which is doing gamedev in a commercially viable and self-sufficient way._

_We've released a few games across Rust, Godot, Unity, and Unreal Engine, and many people played them on Steam. We also made our own 2d game engine with a simple renderer from scratch, and we also used Bevy and Macroquad for many projects over the years, some being very much non-trivial. I've also used Rust full-time at work as a backend developer. This post is not based on a short-sighted opinion of just going through a few tutorials or trying to make a small game for a game jam. We're well over 100k lines of Rust code written over 3+ years._

_The goal of this post is to serve as a reference to dispel commonly said arguments that get repeated over and over. But again, this is a subjective opinion, and in big part being written so that I don't have to continually explain the same things over and over again when people ask. I'd like this to be a reference for why we're likely abandoning Rust as a gamedev tool. We're in no way stopping with game development, we're just stopping with game development in Rust._

_If your goal is to learn Rust because it seems interesting and you like the technical challenge, that's completely fine. Part of what I want to appeal with this post is however how Rust gamedev is often presented, and advice that people often give out to others, without knowing whether they're building a tech demo or attempting to ship something. The community as a whole is overwhelmingly focused on tech, to the point where the "game" part of game development is secondary. As an example of this, I remember one time a discussion around the Rust Gamedev Meetup, that while was probably done jokingly was still imo illustrative of the issue, with something like "someone wants to present a game at the meetup, is that even allowed?" ... I'm not trying to say that people should have the same goals as we do, but I think maybe the way some things are communicated could be clearer, and that people should be more honest about what it is they're doing._

## Once you get good at Rust all of these problems will go away
Learning Rust is an interesting experience, because while many things initially feel like "this is a special problem only I'm having", later one realizes that there's a few fundamental patterns that are universal, and that everyone learning has to re-discover and internalize in order to be productive. This may include simple things like `&str` vs `String`, or `.iter()` vs `.into_iter()` and having to constantly use those, or just the realization of how partial borrows often go against certain abstractions.

Many of these things are just learning pains, and once enough experience is acquired the user can fully anticipate them without thinking and be productive. I've very much enjoyed my time writing various utilities and CLI tools in Rust, where I found it more productive than Python for anything but a few lines of code.

That being said, there is an overwhelming force in the Rust community that when anyone mentions they're having problems with _Rust the language_ on a fundamental level, the answer is "you just don't get it yet, I promise once you get good enough things will make sense". This is not just with Rust, if you try using ECS you're told the same thing. If you try to use Bevy you'll be told the same thing. If you try to make GUIs with whichever framework you choose (be it one of the reactive solutions or immediate mode), you'll be told the same thing. _The problem you're having is only a problem because you haven't tried hard enough._

I believed this for years. I tried, very hard, for years. I've definitely seen this happen on many levels with the language, and I've found myself to be very productive in certain areas, and learned to be able to anticipate what the language and type system wants in order to avoid these issues.

But, and I say this having spent the past ~3 years and written over 100k lines of game-related code in it across the whole ecosystem of frameworks/engines and having made my own, many if not most of the problems don't go away if one isn't willing to constantly refactor their code and treat programming as a puzzle solving process, rather than just a tool to get things done.

The most fundamental issue is that the borrow checker forces a refactor at the most inconvenient times. Rust users consider this to be a positive, because it makes them "write good code", but the more time I spend with the language the more I doubt how much of this is true. Good code is written by iterating on an idea and trying things out, and while the borrow checker can force more iterations, that does not mean that this is a desirable way to write code. I've often found that being unable to just move on for now and solve my problem and fix it later was what was truly hurting my ability to write good code.

In other languages one can write code with "I can throw this away later" in mind, which I've found to be the most useful approach in terms of getting good code. An example being say that I'm implementing a player controller. I just want the player to move and do things, so that I can start building my level and enemies. I don't need a good controller, I just need it to do things. I can surely delete it and make a better one later. In Rust, sometimes just doing a thing is not possible, because the thing you might need to do is not available in the place where you're doing the thing, and you end up being made to refactor by the compiler, even if you know the code is mostly throwaway.

## Rust being great at big refactorings solves a largely self-inflicted issues with the borrow checker
It's very often said that one of Rust's greatest strengths is ease of refactoring. This is most definitely true, and I have had many experiences where I could fearlessly refactor significant parts of the codebase, with everything working afterwards. Everything works as advertised?

The thing is, Rust is also a language that will force the user to refactor much more often than other languages. It doesn't take a lot to suddenly be backed in a corner with the borrow checker and realize "wait I can't add this new thing because things will no longer compile, and there's no workaround other than code restructuring".

This is where experienced people will often say that this becomes less of an issue once you get better at the language. My take is, while that is 100% true, there's a fundamental problem of games being complex state machines where requirements change all the time. Writing a CLI or a server API in Rust is a very different experience than writing an indie game. Assuming the goal is to build a good experience for players rather than an inert set of general purpose systems, the requirements might change from day to day just after having people play the game and you realize some things need to fundamentally change. Rust's very static and overly-checked nature fights directly against this.

Many people would counter-argue that if you end up fighting the borrow checker and have to refactor your code it's actually good, because this makes your code better. I think this is a valid point to raise for something where you know what you're building. But in the majority of cases, I don't want "better code", I want "game faster" so that I can test it sooner and realize if the idea was good. It's not uncommon to be forced to make a choice between "do I break my flow and spend the next 2 hours refactoring this to test an idea, or do I make the codebase objectively worse?".

I'd argue as far as _maintainability being the wrong value for indie games_, as what we should strive for is iteration speed. Other languages allow much easier workarounds for immediate problems without necessarily sacrificing code quality. In Rust, it's always a choice of _do I add an 11th parameter to this function, or add another `Lazy<AtomicRefCell<T>>`, or do I put this in another god object, or do I add indirection and worsen my iteration experience, or do I spend time redesigning this part of code yet again._

## Indirection only solves some problems, and always at the cost of dev ergonomics
One fundamental solution that Rust really likes and that very often works is adding a layer of indirection. A canonical example of this is [Bevy's events](https://bevy-cheatbook.github.io/programming/events.html), which are the _go-to suggested solution for anything related to "my system needs to have 17 parameters to do its thing"_. I've tried to be on both sides of this, even specifically in the context of Bevy of trying to use events more heavily, and trying to just put everything in a single system. That being said, this is just one example.

Many issues with the borrow checker can simply be worked around by doing something indirectly. Or by copying/moving something out, then doing the thing, then moving it back. Or by storing it in a command buffer and doing it later. This can often lead to interesting discoveries in terms of design patterns, for example one thing I found quite neat is how a very large portion of issues can be solved by reserving entity ids ahead of time (e.g. [World::reserve in hecs](https://docs.rs/hecs/latest/hecs/struct.World.html#method.reserve_entity), note the `&world` and not `&mut world`), combined with a command buffer. These patterns are amazing when they work, and they solve otherwise very difficult problems. Another example is a seemingly very [specialized get2_mut in thunderdome](https://docs.rs/thunderdome/latest/thunderdome/struct.Arena.html#method.get2_mut) that seems like a random idea at first, until one realizes that this is something that comes up all the time and that solves many unexpected issues.

I won't get into arguing whether the learning curve to being productive is reasonable. It is certainly not, but this whole post is about the problems persisting on a fundamental level even after enough experience is acquired.

Back to the point, while some of the above can solve specific problems, there's very often going to be a situation that can't be solved with a specialized and well-thought-out library function. This is where solving problems with "just do the problematic thing later" with a command buffer or event queue becomes something many will suggest, and it certainly works.

The problem with games specifically is that we often care about inter-connected events, specific timings, and just overall managing a lot of state at once. Moving data across an event barrier means the code logic for a thing is suddenly split into two part, where even if the business logic might be "one chunk", it has to be cognitively regarded as two.

Anyone who's been in the community long enough have had the experience of being told that this is _actually a good thing_, separation of concerns, code is "cleaner", etc. You see, Rust was designed in a smart way, and if something can't be done, it's because the design is wrong, and it just wants to force you down the right path ... right?

What would be 3 lines of code in C# suddenly becomes 30 lines of Rust split into two places. The most canonical example here is something like: "while I'm iterating over this query I want to check a component on this other thing and touch a bunch of related systems" (spawn particles, play audio, etc.). I can already hear people telling me _well duh, this is obviously an `Event`, you shouldn't be writing that code inline_.

Just imagine the horror of wanting to do something like (Unity code coming, brace yourselves, or just pretend it's Godot):

```
if (Physics.Raycast(..., out RayHit hit, ...)) {
  if (hit.TryGetComponent(out Mob mob)) {
    Instantiate(HitPrefab, (mob.transform.position + hit.point) / 2).GetComponent<AudioSource>().clip = mob.HitSounds.Choose();
  }
}
```

This is a relatively simple example, but it is something one might want to write. And especially when implementing a new mechanic and testing things, it is something that you can just write. There is no maintainability to think about, I just want to do very simple things, and I want to do them in the place where they are supposed to happen. I don't want a `MobHitEvent`, because maybe there's 5 other things I may want to check the raycast against.

I also don't want to check "is there a `Transform` on the `Mob`"? Of course there is one, I'm making a game. All of my entities have a transform. But Rust won't let me have a `.transform`, let alone in a way that would _never_ crash with a double borrow error if I'm accidentally inside queries with overlapping archetypes.

I also maybe don't want to check if the audio source is there. Sure I could `.unwrap().unwrap()`, but the more observant 🦀's will notice the lack of `world` being passed around, are we just assuming a global world? Aren't we using dependency injection to write out our query as another parameter in the system with everything laid out up front? Is `.Choose` assuming a global random number generator? What about threads??? And where exactly is the physics world, are we seriously assuming that to be a global too?

If you're thinking "but this won't scale" or "it might crash later" or "you can't assume global world because XYZ" or "what if multiplayer" or "this is just bad code" ... I hear you. But by the time you finished explaining to me that I'm wrong I've already finished implementing my feature and moved on. I wrote my code in single pass without thinking about the code, and as I was writing it I was thinking about the gameplay feature I was implementing and how it affects the player. I wasn't thinking "what's the right way to get a random generator in here" or "can I assume this being single threaded" or "am I in a nested query and what if my archetypes overlap", and I also didn't get a compiler error afterwards, and I also didn't get a runtime borrow checker crash. I used a dumb language in a dumb engine and just thought about the game the whole time I was writing the code.

## ECS solves the wrong kind problem
Because of the way Rust's type system and borrow checker works, ECS comes up as a naturally occurring solution to the problem of "how do we have stuff reference other stuff". Unfortunately, I think there's quite a bit of terminology mixup, and not only do different people mean different things, but also that the large part of the community attributes some things to ECS that aren't actually ECS. Let's try to separate things out.

Firstly, let's mention a few things which we can't really do for various reasons (there's some nuance, but simplified since this article is already way too long):

* Pointer-y data with actual pointers. The problem here is simple, if character A follows B, and B gets deleted (and de-allocated), the pointer would be invalid.
* `Rc<RefCell<T>>` combined with weak pointers. While this could work, in games performance matters, and the overhead of these is non-trivial due to memory locality.
* Indexing into arrays of entities. In the first case we'd have an invalid pointer, in this case if we have an index and we remove an element, the index might still remain valid, but point to something else.
Now comes a magic solution that gets rid of all these problems, generational arenas, as best shown by [thunderdome](https://docs.rs/thunderdome), which by the way is a library I'd highly recommend as it's small and lightweight and does what it's supposed to while keeping its codebase readable, the last point being quite rare in the Rust ecosystem.

Generational arena is basically just an array, except instead of having our id be an index, it's a tuple of `(index, generation)`. The array itself then stores tuples of `(generation, value)`, and we to keep things simple we can just imagine that every time something is deleted at an index we simply bump up the `generation` counter at that index. Then we just need to make sure that indexing into the arena always checks if the generation of the provided index matches the generation in the array. If the item was deleted, the slot would have a higher generation, and the index would then be "invalid" and act as if the item doesn't exist. There's some other reasonably simple problems to solve, such as keeping a free list of slots where we might want to insert to make insertion fast, but none of that is really that relevant for the user.

The key point being, this allows a language like Rust to completely side-step the borrow checker and allow us to do "manual memory management with arenas" without actually touching any hairy pointers, and while remaining 100% safe. If there was one thing to point at that I like about Rust, it'd be this. Especially with a library like `thunderdome` it really feels that this is a great match, and that this data structure very well fits the language as it was intended.

Now comes the fun part. What most people attribute as benefits of ECS are for the most part benefits of generational arenas. When people say "ECS gives me great memory locality", but their only query around mobs look like `Query<Mob, Transform, Health, Weapon>`, what they're doing is basically equivalent to `Arena<Mob>` where the struct is defined as

```
struct Mob {
  typ: MobType,
  transform: Transform,
  health: Health,
  weapon: Weapon
}
```

Now of course defining things in this way doesn't have all the benefits of ECS, but I feel it should be very explicitly pointed out that just because we're in Rust and just because we don't want to have everything be `Rc<RefCell<T>>` does not mean we need ECS, it could just mean that what we really want is a generational arena.

Back to ECS, there are few different ways to look at ECS that are very different:

*ECS as dynamic composition*, allowing combinations of components to be stored and queried and modified together without having to be tied in a single type. The obvious example here that many people end up doing in Rust (because there's no other good way of doing this) is tagging entities with "state" components. One example might be that we want to query all `Mob`s, but maybe some of them have been morphed into a different type. We could simply do `world.insert(entity, MorphedMob)`, and then in our query we can either query `(Mob, MorphedMob)`, or something like `(Mob, Not<MorphedMob>)` or `(Mob, Option<MorphedMob>)` or check the presence of said component in code. Those might end up doing different things depending on different ECS implementations, but in practice we're using this to "tag" or "split" entities.

Composition can be much richer than this. The example before also fits in this, where instead of having one big struct `Mob`, we can have this be a separate `Transform`, `Health`, `Weapon`, and maybe other things. Maybe a mob without a weapon doesn't have the `Weapon` component, and once it picks up a weapon we insert it into the entity. This would allow us to iterate over all mobs with a weapon in a separate system.

I'll also include Unity's "EC" approach in the dynamic composition, as while it may not be the traditional purist "ECS with systems", it very much uses components for composition, and performance concerns aside, it ends up allowing for very similar things as if it was "pure ECS". I'd also like to give an honorable mention to Godot's node system, where child nodes are often used as "components", and while that has nothing to do with ECS, it has everything to do with "dynamic composition", as it allows nodes to be inserted/removed at runtime, and the behavior of entities to be altered because of this.

It should also be noted that the approach of "splitting up components into as small as possible for maximum reuse" is something that is very often cited as a virtue. I've been in countless arguments where someone tried to convince me how I absolutely should be separating `Position` and `Health` out of my objects, and how my code is spaghetti if I'm not doing that.

Having tried those approaches quite a few times, I'm now very much on the _hard disagree_ side in complete generality, unless maximum performance is of concern, and then I'd concede on this point only for those entities where this performance concern matters. Having also tried the other approach of having "fat components", both before and after the "separated" approach, I feel that the "fat components" approach much better fits into games that have lots of logic that is unique to what is happening. For example, modelling `Health` as a general purpose mechanism might be useful in a simple simulation, but in every game I end up wanting very different logic for player health and enemy health. I also often end up wanting different logic for different types of non-player entities, e.g. wall health and mob health. If anything I've found that trying to generalize this as "one health" leads to unclear code full of `if player { ... } else if wall { ... }` inside my health system, instead of having those just be part of the big fat player or wall systems.

*ECS as dynamic structure of arrays*, where due to how components are stored in ECS we get the benefit of iterating over say just `Health` components and having them next to each other in memory. For the uninitiated, this would mean instead of `Arena<Mob>`, we'd instead have:

```
struct Mobs {
  typs: Arena<MobType>,
  transforms: Arena<Transform>,
  healths: Arena<Health>,
  weapons: Arena<Weapon>,
}
```

and where values at the same index would belong to the same "entity". Doing this by hand is annoying, and depending on your background and which languages you've used in the past you may have had to do this by hand at some point. But thanks to modern ECS, we can kind of get this for free by just writing out our types in a tuple, and have the underlying storage magic put the right things together.

I'd also call this use case *ECS as performance*, where the point of doing things this was is not "because we want composition", but "because we want more memory locality". This may actually have some valid applications, but I'd say for the vast majority of indie games that get shipped, this is not necessary. I'm intentionally saying "that get shipped", because of course it's easy to build mindblowingly complex prototypes that will require this, but those will also be infinitely far from ever being "played" by other people, and thus aren't of concern for this article.

*ECS as a solution to the Rust borrow checker*, which is what I think most people using ECS are actually doing, or rather the reason why they're using ECS. If anything, ECS is a very popular solution and recommendation to give in Rust, because it tends to work around a lot of the issues. We don't need to care about lifetimes of things if we're just passing around `struct Entity(u32, u32)`, since it's all nice and `Copy`, just like Rust likes it.

The reason I have this as a separate point, is that many times people use ECS because it solves the particular problem of "where do I put my objects", without really using it for composition, and without really needing its performance. There's nothing wrong with that, only when such people end up getting into arguments all over the internet trying to convince other people that their approach of doing things is wrong, and that they should be using ECS a certain way because reasons mentioned above, without actually needing it in the first place.

*ECS as dynamically created generational arenas*, which is something I kind of wanted to exist and tried to [hack together](https://github.com/darthdeus/plushy), only to realize that to truly get what I'd want I'd have to re-invent many ugly interior-mutability related things that I wanted to avoid doing in the first place, just to allow doing things like `storage.get_mut::<Player>()` and `storage.get_mut::<Mob>` at the same time. Rust has this nice property that while you do things the way you want it to it's all fun and pretty, but once you want something it doesn't really like, things quickly turn into "I need to re-implement my own `RefCell` that does this specific thing" or worse.

What I really mean by this point is that while generational arenas are nice, one of the big annoying downsides is that one has to define a variable and a type for every arena they intend to use. This can of course be solved by ECS if one just uses a single component in every query, but it would be nice and neat if one didn't need a full archetypical ECS just to get an arena-per-type on demand. There are ways to do this of course, but I'm way past burnout on trying to re-invent parts of the ecosystem to do this myself, and way past caring enough to force myself to do it.

*ECS because Bevy*, which is partly meant as a joke, but I think due to Bevy's popularity and it's all-encompassing approach it should be mentioned as a separate view of ECS. Because for most engines/frameworks, ECS is a choice, it's a library that one decides to use. But as far as Bevy games are concerned, this isn't something optional that is used only for some things, the whole game is ECS.

It should be noted in the most positive way, that while I may disagree on many things, it's hard to deny how much improvement has Bevy done to ECS APIs and the ergonomics of _ECS itself_. Anyone that has seen or even used things like [specs](https://github.com/amethyst/specs) understands how much better Bevy is at making ECS nice to use and approachable, and how much it improved over the years.

That being said, I think this is also the core cause of the problem I have with how the Rust ecosystem views ECS, and especially how Bevy does. ECS is a tool, a very specific tool that solves very specific problems, and that does not come for free.

I'll take a sidestep here and let's talk about Unity for a second. Regardless of what happened with its licensing, leadership or what its business model is, it'd be foolish to think of Unity as anything but one of the main things that made indie gamedev the success that it is. Looking at SteamDB charts there are now almost 44 000 games in Unity on Steam, with the second engine being Unreal at 12 000, and the rest falling well behind.

Anyone that has been following Unity in the recent years knows about Unity DOTS, which is essentially their "ECS" (and other data oriented things). Now as a past, present and future user of Unity, I'm very excited by this, and one of the main reasons I find it exciting is that this co-exists with the existing game object approach. There's many intricacies, but at its core, things are what one could expect. A single game can use DOTS for some things, while also using the standard game object scene tree as it was before, and these two go well together.

I don't think one would find a person in the Unity space who understands what DOTS things and thinks it's a bad feature that shouldn't exist. But I also don't think one would find a person who thinks DOTS is all there should be in the future, and that game objects should be erased from existence, and all of Unity should be moved to DOTS. Even ignoring maintenance and backwards compatibility, this would be remarkably stupid, as there are so many workflows that naturally fit into game objects.

Those that have used Godot can probably see a similar view, especially those that used gdnative (e.g. via godot-rust), where while node trees maybe not the best data structure for everything, they for sure are extremely convenient for quite a few things.

Taking this back to Bevy, what I don't think many people realize, is just how all-encompassing the "ECS everything" approach is. An obvious example and in my opinion a large failure point here is Bevy's UI system, which has been a pain point for a while, especially combined with the "we'll start working on the editor this year for sure!" types of promises. If you take a look at Bevy's UI examples it becomes very quickly obvious that there isn't much there, and taking a look at the source code for something simple as a button that changes color when hovered and clicked quickly reveals why. Having actually tried to use Bevy UI for something non-trivial, I can confirm that the pain is even greater than it looks, as the amount of ceremony required by the ECS to do anything UI related is just completely insane. As a result, Bevy's closest thing that exists to an editor is a 3rd party crate that uses egui. I'm simplifying things a bit, and of course there's more that goes into making an editor than just UI, but I do think that the insistence of putting everything into ECS, including UI, is definitely not helping here.

ECS in Rust has this tendency of turning from something that is considered a tool in other language to become almost a religious belief. Something that should be used because it is pure and correct, and because doing that is the right way.

Programming language communities often have certain tendencies, and having been a serial language hopper over the years I find it interesting to compare these. The closest thing to Rust's view on ECS I can think of is Haskell, except, and I know this is an oversimplification but I'll say it anyway, I do feel that the overall community in Haskell is a lot more mature, and that people in general tend to be more reasonable about the existence of other approaches, and view Haskell as a "fun tool to solve problems where it fits well".

Rust on the other hand often feels like when you talk to a teenager about their preference about anything. What comes out are often very strong opinions and not a lot of nuance. Programming is a very nuanced activity, where one has to often make suboptimal choices to arrive at a result in a timely manner. The prevalence of perfectionism and obsession with "the correct way" in the Rust ecosystem often makes me feel that the language attracts people who are newer to programming, and are easily impressionable. Again, I understand this doesn't apply to everyone, but I think the overall obsession with ECS is in some sense a product of this.

## Generalized systems don't lead to fun gameplay
A very commonly offered solution to many issues preventing here is more generalization through systems. If only components were more granularly split up and proper systems were used, surely all those special-cased problems would've been avoided, right?

Strong argument that is tough to say much against, other than "general solutions lead to boring gameplay". Having been quite active in the Rust gamedev community I've seen a lot of projects others were building, of course often the suggestions they offer do actually correlate with the game they're working on. People who tend to have neatly designed systems that operate in complete generality tend to have games that aren't really games, they're simulations that will eventually become a game, where often something like "I have a character that moves around" is considered gameplay, and where the core focus is on having one or more of the following:

* Procedurally generated world, planets, space, dungeons.
* Voxel based anything, with deep focus on voxels themselves, rendering voxels, world size and performance.
* Generalized interactions where "anything can do X with anything else".
* Rendering in the most optimal way possible, if you're not using draw indirect are you even making a game?
* Having good types and "framework" for building games.
* Building an engine for making more games like the one that is about to be built.
* Multiplayer.
* Lots of GPU particles, the more particles the better the VFX.
* Well structured ECS and clean code.
* ... and many more

_All of these are fine goals in terms of playing around with tech and learning Rust, but I want to re-iterate what was said at the top of this article. I'm not evaluating Rust from the perspective of technical curiosities or "this scratches the right brain itch". I want to make real games that will get shipped to real people (not developers) in reasonable amount of time, that those people will pay for and play, and have an actual chance of hitting the front page of Steam. To clarify, this isn't a cold blooded "make money at all costs" scheme, but it's also not a "I'm just doing this for the lulz". The whole article is written from a perspective of wanting to be a serious game developer who cares about games, gameplay and players, and not just tech enthusiasm._

_Again, nothing wrong with tech enthusiasm, but I think people should be very careful about what their actual goals are, and above all be honest with why they're doing what they're doing. Sometimes I feel like the way some projects present themselves are the way people talk about those projects is false advertising that creates an illusion that commercial goals can be attained with said approaches, instead of making it more clear that "I'm just doing this for the tech itself"._

Now back to generalized systems. Here's a few things that I think create good games, that are going directly or indirectly against generalized ECS approaches:

* Mostly hand-designed playthrough of a level. This does not mean "linear" or "story", but it does mean "lots of control over when the player sees what".
* Carefully crafted individual interactions throughout the levels.
* VFX that are not based on having lots of same-y particles, but time synchronized events (e.g. multiple different emitters firing on a hand-designed schedule) working across all the game's systems.
* Iterated playtesting with multiple passes on gameplay features, experimentation and throwing away what doesn't work.
* Shipping the game to players as fast as possible so that it can be tested and iterated on. The longer nobody sees it, the bigger chance nobody cares about it when it comes out.
* Unique and memorable experience.

I understand that reading this many people would think I'm imagining an artsy-fartsy game made by a painter and not a real programmer who wants to make a game like Factorio, but this isn't true. I still like systemic games, I like code, I want to make something that is driven by programming, because I do feel like I'm mainly a programmer.

What I think most people get wrong is mistaking carefully thinking through player interactions and designing them as something artistic. I'd argue that this is what game development actually is. Game development isn't building a physics simulation, it's not building a renderer, or building a game engine, or designing a scene tree, or a reactive UI with data bindings.

A good example game here would be The Binding of Isaac, which is a very simple roguelike with hundreds of upgrades that modify the game in very involved, interactive and deeply complex ways. It's a game with many systems that play into each other, but it's also something that's not at all generic. It's not a game with 500 upgrades of the "+15% damage" variety, but many upgrades are of the "bombs stick to enemies" or "you shoot a laser instead of projectiles" or "first enemy you kill each level will never spawn again".

Looking at a game like this retrospectively may make it look like it's something you could design up front with general purpose systems, but I think this is also something where most people go completely wrong with game development. You don't make a good game like this by sitting in a dungeon for a year, thinking through all the edge cases and building a general system and then PCGing all the upgrades. You build a prototype with some small amount of mechanics and have people play it, see that the core things work, and then add more things and have people play again. Some of these interactions have to be discovered through deep knowledge of the game after playing a lesser version of the game for many hours and trying many different things.

Rust is the type of language where wanting to do a new type of upgrade might lead you down a path of refactoring all of the systems, and many would even say "that's great, now my code is much better and can accommodate so many more things!!!". It sounds like a very convincing argument, one that I've heard many times, and one that has also caused me to waste a lot of time chasing down solutions to the wrong problems.

A more flexible language would allow the game developer to immediately implement the new feature in a hacky way, and then play the game, test it and see if the feature is actually fun, and potentially do a bunch of these iterations in a short amount of time. By the time the Rust developer is finished with their refactoring, the C++/C#/Java/JavaScript developer has implemented many different gameplay features, played the game a bunch and tried them all out, and has a better understanding of which direction should their game be taking.

Jonas Tyroller explains this extremely well in his video on game design as a search, which I'd 100% recommend every game developer to watch, because it feels like the best explanation of why so many games people make (myself included) are profoundly terrible. A good game is not made in a lab where careful types are crafted, it is made by a developer who is a grandmaster player at the genre, and who understands every aspect of the design and has tried and failed many things before reaching upon the final design. A good game is made through scraping a lot of bad ideas, through a non-linear process.

## Making a fun & interesting games is about rapid prototyping and iteration, Rust's values are everything but that
To better define this point we have to define what is meant by "game development" in this article. We're not talking about AAA, or large scale very long term projects in general. I don't think anyone can realistically think they're going to build a successful 5 year game project unless they already have a lot of prior experience with both game development and the tooling they're using. We're talking about indie games made by individuals or small teams on relatively tight budgets/timelines.

Secondly, there are many reasons one could make a game, but our intention is to make something other people will play and consider to be good, without knowing which technology/engine/framework/ideology was used to create it, without knowing or relating to the author, and without having any prior exposure. I feel like this especially needs to be stressed out, because while the Rust community is overall very supportive, it often creates a very false idea that "this is so cool, people will love a game like this". It's not a problem only Rust struggles with, and many gamedevs end up showing their games to other gamedevs and gamedev communities, and thus fall for the same fallacy.

Because of the general vibes in the Rust community it's very common for people to receive very positive reinforcement on what they're building. This is nice in terms of mental health and short term motivation, but having gone through the process of releasing something on Steam publicly more than once, I feel like many people are headed for a bitter realization once people who aren't in their friend group/community see their game. The reason I'm saying this is that I think the community as a whole has adopted this idea of relentless positivity and praise towards everything Rust related, shielding itself completely from the outside world.

But the real world of gamers is not as nice. Gamers on Steam don't care if something is made in Rust, they don't care if it took 5 years to make, they don't care if the code is opensource. They care about looking at the game, and within a few seconds being able to tell if this is going to be a waste of time, or something potentially interesting.

I've seen many people dismiss these things as the young generation and attention spans and ADHD this/that and people should appreciate XYZ. I don't find any of these views helpful, because everyone does this, we as game developers are just biased when it's about our games. When you're shopping at a grocery store and look at bananas and some of them have a slightly ugly color or look a bit damaged you'll pick the ones that look better instead. When going to a restaurant you'll pick one that looks like they'll have good food at a good price, or at least delivers an experience you care about.

I'd even say that it is correct and desirable that players do not care about the developer and just look at the game for a few seconds, but at least that keeps us honest. It keeps the games be about the game itself and nothing else, because ultimately, it is the game and the experience of playing it that matters.

It also reveals the values one as a game developer should appeal to. If you're showcasing your game and the response is anything but "can I please play this?", the game was not interesting to the person who you showed it to. At least not in the sense that truly matters for the purposes of making commercially successful games.

People would often argue that Rust appeals to values like "maintainability" and how this leads to better games that don't crash, but I think the problem here is completely different scales. Surely we can all agree that a game crashing when someone presses play is bad, and it is definitely bad when you corrupt a save file and the player loses progress.

But I think all of this completely misses the point of what matters to players. There are many cases where people would get their progress wiped and they'd still come back to the game and play it again, because the game was that good. I've done this more than once as a player.

Rust as both language and community is so preoccupied with avoiding problems at all cost that it completely loses sight of what matters, delivering an experience that is so good that whatever problems are there aren't really important. This doesn't mean "ship crap games", it means focusing on the game being a good game, not on the code being good code.

## Procedural macros are not even "we have reflection at home"
Game development as a domain of programming often requires one to write more than one type of code. We have system-y code for things like collisions, physics, particles. We have gameplay code for "scripting" entity behaviors. We have UI, VFX, audio. And then we also have tools. Depending on the game that is being built the size of each category may vary, but after working on enough of different genres of games I'd say it's generally universal that some amount of effort will have to be spent on every aspect.

Rust fits very nicely in the low level algorithmic areas where one knows exactly what the problem is and just needs to solve it. Unfortunately, a lot of gamedev requires more dynamic approaches, and this becomes especially painful around level editing, tooling and debugging.

Even something as simple as "print this object" is not a problem that can be reasonably solved without either writing code, or creating procedural macros. Now many languages have macros, and for those who haven't used Rust for long enough, they might not know that there's two types of macros in Rust:

* Declarative macros: These are relatively simple to create and very useful, but unfortunately quite limited. As many things in Rust, "safety" is above all else, and things that would be completely fine in C preprocessor macros become an impossible issue. The simplest example here is concatenating tokens, which now has a famous paste crate that gives a partial solution using a procedural macro. At a surface level you'd think great, problem solved, right? ... but unfortunately not even close, for example things like nesting and mixing procedural and declarative macros together isn't always going to work, and it's not even obvious what's possible and why until a lot of time is spent on figuring out the technicalities.
* Procedural macros: As a core idea procedural macros basically allow the programmer to run code at compile time, consume Rust's AST, and generate new code. There are many issues with this unfortunately. Firstly, proc macros aren't really cached and get re-run on recompiles. This ends up forcing your code to be split up in multiple crates, which isn't always possible, and if you rely on proc macros more heavily your compile times will suffer by a huge amount. There's many convenient proc macros like profiling's function macro which are very very useful, but ultimately unusable, because they destroy incremental build times. Secondly, procedural macros are incredibly difficult to write, and most people end up using very heavy helper crates, such as syn, which is a very heavy Rust parser that eagerly evaluates everything it's applied to. For example, if you want to annotate a function and just parse its name in your macro, syn will end up parsing the whole function body regardless. There's also the case where the author of syn is also the author of serde, a popular Rust serialization crate, which at some point last year started shipping a binary blob with its installation in a patch release, rejecting the community backlash. This isn't really a case against Rust, but I feel it should be mentioned, because it shows how a big part of the ecosystem is built on libraries made by single developers who can make potentially dangerous decisions. Of course this can happen in any language, but in terms of procedural macros this is very important, because almost everything in the ecosystem uses crates made by this specific author (syn, serde, anyhow, thiserror, quote, ...).
Even ignoring the above, procedural macros have a very steep learning curve, and they have to be defined in a separate crate. This means that unlike with declarative macros where you can just create one as if you were making a function you can't easily just make a new procedural macro.

In contrast, using reflection in C# is extremely easy, and if performance is of no concern (which it often isn't in cases where reflection is used) it can be a very quick and useful option for building tools or debugging. Rust doesn't offer anything of the sort, and the last approach for compile time reflection has been basically cancelled in one of last year's Rust dramas.

As this article aims to remain technical I don't see much value in explaining the drama in detail or trying to take sides, because while all those are of varying importance to different people, practically the consensus in the community is that there is no more compile time reflection in sight in the near future, which is incredibly sad for everyone involved with the language. Procedural macros are a big and powerful tool, but their utility for indie game development is incredibly low, as their development cost and complexity is a bit too high to be used to solve minor issues that could've been solved by reflection with little to no effort.

## Hot reloading is more important for iteration speed than people give it credit for
Before we get into Rust and hot reloading, I'd like to mention a few things.

Firstly, if you haven't seen Tomorrow Corporation Tech Demo, I would 100% recommend every single game developer to watch this video to see what is possible in terms of hot reloading, reversible debugging, and overall tooling for game development. If you think you know what these things are, watch the video anyway. I have long felt that hot reloading was important at least to some extent, but seeing what these guys have built on their own really makes me feel ashamed of ever feeling that certain workflows were adequate for developing interactive experiences.

For those who haven't watched the video, here's what the guys at Tomorrow Corporation have done:

* Built their own programming language, code editor, game engine, debugger, and games.
* Built support for hot reloading across the whole stack.
* Reversible time-travel debugging with a timeline that can scrub across game states.
* ... just watch the video :) I promise you won't regret it

I understand that building something like this into an existing platform like .NET, or into a native language like C++ or Rust is borderline impossible in complete generality, but I also refuse the argument that just because it's hard and won't work 100% we shouldn't strive to want these things.

There are many existing platforms/languages that support hot reloading to various extents. During my exploration I went as far as making a game in Common Lisp in order to get a feel for its hot reloading capabilities. I wouldn't necessarily advise people do that, but one does not have to go that far.

Since .NET 6, it is now possible to do hot reloading in any C# project. Now I've heard people report mixed experiences on this, but I've also tried it myself, and it's a bit tough for me to take some of the arguments seriously, especially when they're from more recent times and not from "I tried it when it came out". In the context of Unity, there now is hotreload.net, which is a custom implementation made specifically for Unity, which I've been using for about 4 months now, and which has been completely amazing in terms of productivity. This is actually the #1 reason we're moving back to Unity. It's not the reason we're abandoning Rust, but it is a reason we're going to Unity and not Godot or UE5. (At the time of writing Godot does not support .NET hot reload, and UE still only has blueprints and C++.)

For the purposes of this article, we can just focus on hot reloading bodies of functions, that is the only valid operation would be changing code inside of a function, and hot reloading that. Somehow this is a controversial topic in the Rust ecosystem, and many people will happily argue that it's not useful if it doesn't do everything, or that it's too restricted to be useful, or that the potential for bugs outweighs any possible benefits.

I have a very hard time emphasising with any of this in the context of game development. Games are anything but stateless data processors.

Few cases where hot reloading becomes incredibly useful:

* Immediate mode anything, be it UI or drawing. Even with fast compile times the iteration speed is significantly improved as one doesn't have to constantly re-enter the same state.
* Debugging with immediate mode drawing/geometry. This is probably my favorite use case, especially around debugging character controllers and physics, where I might enter an unexpected/buggy state, and with hot reloading I can simply add a line few lines to draw the relevant values in-game to see what's happening without having to reproduce the issue again.
* Tweaking constants that affect gameplay. While in some cases restarting the game with a new value would lead to a different result, games aren't scientific experiments. We don't need reproducibility, we need fun. It's much easier to optimize for fun when I can tweak values while playing. Crates like inline_tweak are useful here, but they require foresight. Hot reloading allows me to work on an unrelated feature and randomly get an idea "I wonder what if" and just do it, without it being a detour in what I was doing before.

It should be noted that Rust does in fact have a solution in the form of hot-lib-reloader, but having tried it it's nowhere near perfect, even for the very simple use case of just reloading functions. I've had it break on many random occasions, and ultimately gave up as it was causing me more effort to play around with it than it was saving. Even if this crate worked without any issues it doesn't solve the issue of randomly tweaking things, as it still requires planning and foresight, which reduces potential creative usage.

Many people counter hot reloading with "but the compiler does XYZ", to which I'd love to suggest something that would never get merged, but would be nice to have. What if there was a compiler flag ... and yes, I can already see people scream "the poor compiler team" ... I guess we'll never have this.

There many partial workarounds, but none of them get close to the utility of true hot reloading, which is what I'd call what .NET and Unity currently have. Scripting languages are a partial solution and problematic in Rust for many reasons, manually implemented hot reloading with dylibs is limited, and any form of state serialization and restarting again only works for big code changes, and not just tweakability. Not to say these things aren't useful, but I think we as game developers should desire higher level of tooling than just "I can reload a few structs in my code", especially when other mature platforms can support much more general workflows.

## Abstraction isn't a choice
This section is motivated by a very simple code sample I just wrote while working on our game. I have a UI with a list of characters, and a detail page that appears when a character (duck) is selected.

The way the UI is structured I just have helper functions for each state of the UI, as we're using egui and immediate mode requires most things to be available in most places. This actually works great, because things like this will work

```
egui::SidePanel::left("left_panel").frame(frame).show_inside(
    ui,
    |ui| {
        ui.vertical_centered(|ui| {
            character_select_list_ducks(egui, ui, gs, self);
        });
    },
);

egui::SidePanel::right("right_panel").frame(frame).show_inside(
    ui,
    |ui| {
        character_select_recover_builds(ui, gs, self);
    },
);

egui::TopBottomPanel::bottom("bottom_panel")
    .frame(frame)
    .show_inside(ui, |ui| {
        character_select_missing_achievements(
            egui, ui, gs, self,
        );
    });
```

But let's say some of these have conditional state, and their implementation is actually quite non-trivial. This is the case when selecting a specific duck. Initially, my code was the following

```
egui::CentralPanel::default().frame(frame).show_inside(
    ui,
    |ui| {
        character_select_duck_detail(ui, gs, self);
    }
});

fn character_select_duck_detail(..., state: ...) {
	if let Some(character) = state.selected_duck {
	    // some UI
	} else {
		// other UI
	}
}
```

This again works fine, problem is egui will often require very deep nesting just because almost every layout operation is a closure. It would be very nice if we could reduce the nesting and move the if outside. As a result we'd also separate out two clearly separate things ... first instinct:

```
if let Some(character) = &self.selected_duck {
    character_select_duck_detail(.., character, self);
} else {
   character_select_no_duck(...);
}
```

But here we get slapped on the wrist, did I actually think I could get away with passing self around while also borrowing a field on self?

Even years into using Rust I still sometimes use too much of my brain thinking about the UI or game, and too little thinking about how I should be structuring my code, and end up with a problem like this. The Rust-y instinct would say "clearly you need to separate your state and not pass around a big struct", but this is a great example of how Rust clashes with the most natural way of doing things.

Because in this case we're building a single UI window. I don't want to spend any of my brain cycles thinking about what parts of the UI need what parts of the state, I just want to pass my state around, it's not that big. I also don't want to spend extra time passing around more fields when I add more fields 15 minutes down the line, which I'm almost certain I'll do. I also don't want to be separating things into more than one struct, because there's more than one thing I might want to do an if on, and having gone down the "splitting structs" path before, it rarely works out on first try.

The solution? As many things in Rust, we feel a bit of the 🤡 emotion (clown emoji for those who don't have the right installed), and then change the code to this:

```
if let Some(character) = &self.selected_duck.clone() {
    character_select_duck_detail(.., character, self);
} else {
   character_select_no_duck(...);
}
```

Everything now works, the borrow checker is happy, and we're cloning a string every frame. It won't show up in the profiler, so it really doesn't matter in the grand scheme of things. But it's especially sad for a language that aims to be so fast and optimal to have to resolve to wasting cycles on re-allocating memory more often than one would like, just to stay productive.

I only mention this specific case because it's quite indicative of my overall experience writing Rust, and where many problems are simply solved by extra copying or cloning. It's something most Rust developers are familiar with, but that came as a surprise to many people I was helping learn Rust. Their usual response is "wait I thought Rust was supposed to be very fast and efficient" ... all one can say to that is "oh it is fast, don't worry, in this case cloning the string every frame is totally harmless" and then feel the 🤡 emotion again.

## GUI situation in Rust is terrible
Just like there's a running joke of Rust having 5 games and 50 game engines, we probably need another joke for GUI frameworks. People are trying many different approaches, which in the complete generality of Rust as a language makes sense. But in this article we're focusing on gamedev, and I feel like this is something where we're not only seriously lacking, but I don't even see a way out.

Now when I say UI, I don't mean UI to build an editor, I mean in-game UI specifically. Something that has to be highly stylized and visual. At least in my experience, the hardest part about building game UI isn't figuring out how to do data binding, or how to make things reactively update, or even how to best describe my layout. It is customizing the look and feel of the UI.

This doesn't even touch on things like particles in UI, or various effects the user might want. Obviously a GUI library that is completely agnostic of everything can't have fancy shader effects and particles, but I think that's also part of the overall issue in approach. GUI libraries push all of this onto the user to figure out, and then every user is left to re-invent the wheel in their own framework/engine of choice.

We ended up doing the majority of our UI in egui, which while sub-optimal and confusing in many ways at least provides a decent Painter interface for completely custom UI.

When mentioning this and saying how much better the UI situation is in Unity or Godot people always say something like oh I tried Unity, it was terrible, I'm so much happier doing things in pure code. A very common response, and one that I also used to say, which completely misses the point that building a UI is a skill, and doing so in a complex UI toolkit like Unity or Godot provide is complex and annoying because it is something that has to be learned.

## Reactive UI is not the answer to making highly visual, unique and interactive game UI
There are many GUI libraries in Rust, with many different approaches. Some are bindings to existing GUI libraries, some are immediate mode, some are reactive, and some even retained mode. Some try to use flexbox, while others don't really deal with layout on a fundamental level.

The problem is that as far as game development is concerned, I'm not really sure if we have anything that approaches things the correct way. The reason we have so many libraries is the same reason we have so many game engines, it's because very few people in the Rust ecosystem are actually making games.

At least in my view, game GUI doesn't really care that much about data being updated the fastest, about having reactive re-rendering, data bindings, or the fanciest declarative way to describe a layout.

What I'd want instead is to have a very pretty GUI, with lots of custom sprites, animations, vector shapes, particles, effects, flashes, etc. I want my button to wiggle when it's clicked, I want my text to animate as it's hovered, I want to be able to use a custom shader and distort it with a noise texture. I want particles to fly around when a character box is selected.

I understand that some games might want to render a table with a million elements, but I don't think that should be a goal of a game GUI. I also understand many if not all of the ones linked above are not marketing themselves as a game GUI, but that is partly my point in this section.

As far as I'm aware, there isn't a single solution in the Rust ecosystem that would make it its goal to "be good at making game GUIs". I understand that having something like "particles and shaders" in a GUI is not going to be easy for a library that probably wants to be engine-agnostic, but this again might be another reason for why the situation is unlikely to improve.

I do think that most games want to have buttons that wiggle, text that is animated, boxes that rotate in all the weird ways, and maybe even some kind of ungodly blur effect for when that happens. Is that crazy to want such things?

## Orphan rule should be optional
This section can probably be quite short, because I think anyone who's tried to write a decent amount of userland Rust will feel the pain of orphan rule. It's a great example of something I'd call "muh safety", a desire for perfection and complete avoidance of all problems at all costs, even if it means significantly worse developer ergonomics.

There are mostly valid reasons for wanting the orphan rule for things such as libraries uploaded to crates.io, and I am willing to concede that crates published there should obey this.

But I have a very hard time caring about this rule for applications and libraries developed in end products. I'm explicitly not saying binary crates, because most bigger projects will be composed of more than one crate, and many will be more than one workspace.

Practically, I'd say this should be something we could disable even for published libraries, as some are not really libraries that are consumed by further downstream libraries. Game engines and frameworks are a good example of this, as people using libraries like Macroquad or Comfy really don't need those to uphold the orphan rule in their codebase. It'd be very beneficial for "framework-y" libraries to be able to extend existing things without forking, and provide more unified experience to end users.

But unfortunately, like many things in Rust, "perfection is only in the absolute", and just because there is chance that someone could possibly implemented a conflicting trait we must prohibit this for everyone in every circumstance with no option to disable it.

## Compile times have improved, but not with proc macros
It's been a few years since Rust had truly terrible compile times, and the situation as a whole has certainly improved, at least on Linux. Incremental builds on Windows are still significantly slower, to the point where we initially ended up migrating to Linux (3-5x difference), but alas, at least after purchasing a new high end desktop it only takes a few seconds to build our 10k LoC codebase.

That is, after having spent extensive amounts of time optimizing compile times, removing proc macros, and moving things into their respective crates.

As a good example here, the only reason comfy-ldtk exists is to wrap a single file and ensure serde's monomorphisation happens in a separate crate. This might seem like a petty detail, but at least on my desktop this has resulted in incremental times of +10s instead of just 2s on Linux. A pretty gigantic difference for 1600 lines of struct definitions.

Now I understand, serialization isn't a trivial thing, and I understand serde has a lot of feature. But I also don't think there's any universe where paying 8 second to compile 1600 lines of code is anywhere near reasonable. Especially when you look at the code and see it's all just simple structs. There's no complex generic magic, all of this comes down to serde being slow.

I've seen many people not care about things like this, and having personally raised the issue of incremental compile times many times in many different contexts, and there's always a decent chunk of people who will convince me that it's fine, that their build takes 20-30 seconds or longer and that they're still being productive.

At the risk of angering some, I can only attribute this to lack of experience with better tooling, or simply their game not having reached a stage where they actually need to iterate quickly. Or at the very least, I feels like some people realize how much more polish could their games have if their compile times were 0.5s instead of 30s. Things like GUI are inherently tweak-y, and anyone but users of godot-rust are going to be at the mercy of restarting their game multiple times in order to make things look good. If your experience here differs, I'd love to see an example of a very well polished and non-trivial amount of GUI that was built with a +30s incremental build time.

## Rust gamedev ecosystem lives on hype
It's no news that the Rust gamedev ecosystem is young. When you ask around inside the community most people will admit this when issues are mentioned, and I'd say at least in 2024 we don't have an awareness issue as much anymore.

I would say that the outside world has a very different view though, and I will attribute this to very good marketing on the side of Bevy and a few others. Just a few days ago Brackeys released their video about coming back to gamedev to do Godot development. As I've been watching this and started hearing about all the amazing opensource game engines I already had a feeling. At around 5:20 a picture of a Game Engine Market Map is shown, and I can only say I was truly shocked by seeing three Rust game engines there, and specifically which three: Bevy, Arete and Ambient.

Now I want to make this extra clear, this blog post is not an attempt to take a stab at any specific project, and I understand those projects are not responsible for what other people are doing with their videos. But at the same time, this has become such a theme, or maybe even a meme, in the Rust world, that I feel it should be talked about.

The way the Rust ecosystem generally works is whichever project can make the most amount of promises, shows the best website/readme, has the flashiest gifs, and most importantly appeals to the right abstract values, gets widely praised, regardless of the usability of said project. Then there are other projects which are often under the radar, because they're not sexy and are not promising undeliverable features, but instead are just trying to do a thing in a way that works, and those end up almost never being mentioned, or when they are they're mentioned as second class choices.

The first example here is Macroquad, which is a very practical 2D game library, which runs on basically all platforms and has very simple API, compiles incredibly fast and has almost no dependencies, and was built by a single person. There's also an accompanying library miniquad which provides a graphics abstraction on top of Windows/Linux/MacOS/Android/iOS and WASM. Macroquad has however committed the one of the highest crimes in the Rust ecosystem, and that is using global state, and even being potentially unsound. I say potentially, even thoughI understand that purists will say "no this isn't a question, it is wrong", because for all intents and purposes it is completely safe to use unless you decide to use the lowest level API to touch the OpenGL context. Having used Macroquad for almost 2 years now, I've never ran into this being an issue. It is however something that will forever be mentioned whenever it is suggested, because it does not appeal to the ultimate Rust value, 100% safety and correctness.

The second example is Fyrox, which is a 3D game engine with an actual full 3D scene editor, animation system, and seemingly everything needed to make a game. This project was also made by a single person, who is also making a full 3D game in said engine. Personally I have not used Fyrox, because just like this section mentions, I've been personally guilty of falling for the hype and picking projects that have pretty websites, lots of github stars, and present themselves a certain way. Fyrox has been gaining some traction on reddit lately, but it is truly sad for me how it almost never gets mentioned in any videos, despite having a full editor, which is something Bevy has been repeatedly promising for years now.

The third example is godot-rust, which are Rust bindings to the Godot Engine. The most serious crime committed by this library is that it's not a pure Rust solution, but instead just bindings to a filthy C++ engine. I'm exaggerating a bit, but those that are looking at Rust from the outside may be surprised how close to reality this sometimes is. Rust is pure, Rust is correct, Rust is safe. C++ is bad and old and ugly and unsafe and complex. That's why in Rust gamedev we don't use SDL, we have winit, we don't use OpenGL, we have wgpu, we don't use Box2D or PhysX, we have rapier, we have kira for game audio, we don't use Dear ImGUI, we have egui, and above all we surely can't use an existing game engine that's written in C++. That would be a violation of the sacred crab code that everyone who uses rustup default nightly to get faster compile times agrees on in the license (the same one that prohibits us from using the logo (tm)(c) officially endorsed by the Rust foundation).

If anyone is actually serious about making a real game in Rust, especially in 3D, my #1 recommendation would be to use Godot and godot-rust, because at least they end up having a fighting chance of delivering all the features they need to, because they can lean onto a real engine to help them deliver. We spent a year building BITGUN with Godot 3 and gdnative using godot-rust, and while the experience has been painful in many ways, it wasn't the fault of the bindings, but rather trying to mix large amounts of GDScript and Rust in all the possible and dynamic ways. This was our first and biggest Rust project and what lead us down the Rust path, and ultimately I'd say every game we made using Rust afterwards was less of a game, simply because we spent a lot of time trying to figure out irrelevant technical issues with Rust-the-language, some part of the ecosystem, or just some design decision that ended up being difficult to solve because of the rigidity of the language. I'm not going to say GDScript and Rust interop was easy, it was definitely not. But at least there was the option of "just do the thing and move on" provided by Godot. I feel this is something most people who try code-only solutions don't value, especially in Rust where the language can get in the way of creativity in so many inconvenient ways.

I don't have much to say about Ambient because it is fairly new, and I have not used it, but again, I don't know of anyone else who has used it, and yet it made it into Brackeys video.

Arete came out a few months ago with version 0.1, and actually received a relatively negative response from the Rust community due to being very vague about its claims and being closed source at the same time. Despite that, I've seen it mentioned by outsiders on many occasions, often with very bold claims.

As far as Bevy is concerned, I do believe it being showcased as the "main" Rust game engine is mostly justified, if anything just because of the scale of the project and the number of people involved. They have managed to build a remarkably large community, and while I may disagree with their promises and some choices of the leadership, I can't deny the fact that Bevy is popular.

The purpose of this section is nothing but to bring some awareness to the strange state of things, where outsiders will often just look at how well is each engine marketing itself and what it says in their announcement blog posts. The reason I feel the need to mention all these things, is because I've followed this path more than once, and more than once saw very convincing things people would say, only to later realize that they're just very good at talking, but not as good at delivering on those features.

One notable mention that isn't a game engine is rapier, a physics engine that is very often recommended, as it promises to be a pure Rust solution to physics, a great alternative to the ugly outside world of Box2D, PhysX, and others. After all, Rapier is written in pure Rust, and thus enjoys all the benefits of WASM support, while also being blazingly fast, parallel at its core, and of course very safe ... right?

My experience here mostly comes from 2D, where while basic things do work, some of the more advanced APIs are fundamentally broken, for example convex decomposition crashing on relatively simple data, or multibody joints causing a crash when they're removed. The latter being especially funny, because this makes me feel like I was the first person to try to remove a joint, which doesn't seem like such advanced usage. These might seem like edge cases, but overall I've also found the simulation to be quite unstable, to the point where I ended up writing my own 2D physics engine, and at least in my testing found it to cause less issues on simple things like "prevent enemies from overlapping".

This isn't an ad for my physics library, please don't use it, as it's not very well tested. The point is that if a newcomer to Rust asks for a recommendation for physics, they will be recommended rapier, and many will say it's a great and popular library. It also has a nice website and is widely known in the community. Having been that person and having really struggled for months and thinking it must be me, I must be the one doing something wrong the only reason I feel like I "found out" was because I tried to re-implement it myself.

A lot of the Rust ecosystem has a property of making the user feel like they're doing something fundamentally wrong, that they shouldn't be wanting to do a certain thing, that the project they want to build is undesirable or incorrect. It's a feeling similar to using Haskell and wanting to do side effects ... it's just not a thing "you're supposed to want".

Except in Rust's case, the problem is that very often libraries that end up causing the user to feel this way will get universal praise and recognition, because most of the ecosystem lives on hype, rather than shipped projects.

## Global state is annoying/inconvenient for the wrong reasons, games are single threaded.
I know that just by saying "global state" I'm immediately triggering many people who have strong opinions on this being wrong. I feel this is one of those things where the Rust community has created a really harmful and unpractical rules to put on projects/people. Different projects have vastly different requirements, and at least in the context of game development I feel that many people are mis-judging what are the actual problems. The overall "hate" towards global state is a spectrum, and most won't be arguing 100% against it, but I still feel there's many things where the whole community is just going in a wrong direction. Just to reiterate, we're not talking about making engines, toolkits, libraries, simulations, or anything of the sort. We're talking about games.

As far as a game is concerned, there is only one audio system, one input system, one physics world, one deltaTime, one renderer, one asset loader. Maybe for some edge cases it would be slightly more convenient if some things weren't global, and maybe if you're making a physics based MMO your requirements are different. But most people are either building a 2D platformer, a top down shooter, or a voxel based walking simulator.

Having actually tried the pure approach where everything is injected as parameters multiple times over the years (starting with Bevy 0.4, up to 0.10), and having tried building my own engine where everything is global and playing a sound is just play_sound("beep"), my stance on what is more useful is quite clear.

This isn't meant to be specifically against Bevy, I do think a large part of the ecosystem is guilty of this, with the only exception being macroquad, but I'm using Bevy as an example because it sits on the other end of the spectrum where everything is passed around explicitly.

Here's some things I found very useful to have in Comfy that I use all the time in our games, that make use of global state:

* `play_sound("beep")` for playing one off SFX. If more control is needed, one can use play_sound_ex(id: &str, params: PlaySoundParams).
* `texture_id("player")` for creating a TextureHandle to refer to an asset. There is no asset server to pass around, because at worst I could use paths as identifiers, and since paths are unique, obviously the identifiers will be too.
* `draw_sprite(texture, position, ...)` or `draw_circle(position, radius, color)` for drawing. Since every non-toy engine will batch draw calls anyway, it's not like any of these would do much more than just push a draw command into a queue somewhere. I'm more than happy to have a global queue, because why would I care about passing around anything just to push a "draw circle" into a queue.

If you're reading this as a Rust developer who isn't necessarily a game developer, you might be thinking "but what about threads???", and yes, this is also where Bevy servers as a good example. Because Bevy asked this question and tried to answer it in the most general way possible, what if we just made all our systems run in parallel.

This is a neat theoretical idea, and might seem appealing to many who are new to gamedev, because just like in backend land where things are all async and run on threadpools it might seem this would lead to free performance.

But unfortunately, I feel this is one of the biggest mistakes Bevy has made, and having been asking about this I feel many are starting to realize it too, although few really admit it. Bevy's parallel systems model is so flexible it doesn't maintain consistent ordering even across frames (at least last time I checked). If one wants to maintain ordering, they should specify a constraint.

This again seems reasonable at first, but having tried to make a non-trivial game in Bevy on more than one occasion (months of dev time, tens of thousands of lines of code), what ended up happening is the user ends up specifying a ton of dependencies anyway, because things in a game tend to need to happen in a specific order in order to avoid stuff being randomly delayed by one frame depending on what runs first, or even worse things just sometimes behaving weird because you got AB instead of BA. When you raise an issue about this, you'll be heavily argued against because what Bevy does is technically correct, but for the purposes of actually making a game ends up being a huge amount of pointless ceremony.

Now surely there must be an upside to this? Surely, all of this free parallelism is useful and makes games run blazingly faster?

Unfortunately, after all the work that one has to put into ordering their systems it's not like there is going to be much left to parallelize. And in practice, what little one might gain from this will amount to parallelizing a purely data driven system that could've been done trivially with data parallelism using rayon.

Looking back at all of gamedev over the years, I've written a lot more parallel code in Unity using Burst/Jobs than I have ever achieved in Rust games, both in Bevy and in custom code, simply because most of the work on games ends up being the game, with enough mental energy left to solve interesting problems. While in almost every Rust project I feel most of my mental energy is spent fighting the language, or designing things around the language, or at least making sure I don't lose too much developer ergonomics because something is done in a specific way because Rust requires it to be that way.

Global state is a perfect example in this category, and while this section is long, I feel like it really has to be explained a bit further. Let's begin by just defining the problem. In Rust as a language, there's generally a few options:

* `static mut`, this is unsafe, meaning every usage needs unsafe, which gets very ugly and in the case of accidental misuse leads to UB.
* `static X`: AtomicBool (or AtomicUsize, or any other supported type) ... a decent solution that while a bit annoying at least isn't too annoying to use, but only works for simple types
* `static X: Lazy<AtomicRefCell<T>> = Lazy::new(|| AtomicRefCell::new(T::new()))` ... this ends up being necessary for the majority of types, and is not only annoying in terms of defining it and using it, but also leads to potential crashes at runtime due to double borrows.
* ... and of course "just pass it around, don't use global state"

I can't count the number of cases where I've accidentally caused a crash because of a double borrow on something, and not because the code was "poorly designed to begin with", but because something else in the codebase forced a refactor, and as I was refactoring I ended up needing to also restructure my use of global state, leading to unexpected crashes.

Rust users would say that this means my code was doing something wrong and that it actually caught a bug for me, and that this is a good example of why global state is bad and should be avoided. This isn't completely false, and there are bugs that can happen and that would be prevented by this sort of checking. But practically speaking, and in terms of the types of errors I run into when using a language with easy global state like C#, I'd say that in the context of gamedev it's quite rare that any of these problems actually occur in real code.

On the other hand, crashes due to double borrows when doing anything with dynamic borrow checking are something that can happen very easily, and very often for the wrong reasons. One example being queries on overlapping archetypes with ECS. For the uninitiated, something like this is going to be a problem in Rust (simplified a bit for readability):

```
for (entity, mob) in world.query::<&mut Mob>().iter() {
  if let Some(hit) = physics.overlap_query(mob.position, 2.0) {
    println!("hit a mob: {}", world.get::<&mut Mob>(hit.entity));
  }
}
```

The problem being, we're touching the same thing from two different places. An even easier example would be iterating over pairs by doing something like this (again simplified)

```
for mob1 in world.query::<&mut Mob>() {
  for mob2 in world.query::<&Mob>() {
    // ...
  }
}
```

Rust's rules prohibit having two mutable references to the same object, and anything that could potentially lead to this can't be allowed. In the above cases we'd get a runtime crash. Some ECS solutions work around this, e.g. in Bevy one can at least do partial overlaps when the queries are disjoint, e.g. Query<(Mob, Player)> and Query<(Mob, Not<Player>)>, but that only solves the case where nothing overlaps.

I'm mentioning this in a section on global state, because the existence of such limitations becomes especially apparent once things are made global, because it becomes very easy to accidentally touch a RefCell<T> that another part of the codebase is touching through some global reference. Again, Rust developers will say this is good, you're preventing a potential bug!, but I'll again defer to saying that I don't think I've felt many cases where this has actually saved me from doing something wrong, or where doing this in a language without such restrictions would cause an issue.

There's still the question of threading, but I think the main fallacy is where Rust game developers assume that games are the same as backend services where everything must run async in order to perform well. In game code one ends up having to wrap things in a Mutex<T> or AtomicRefCell<T> not to "avoid issues they'd run into otherwise if they were writing C++ and forgot to synchronize access", but rather just to satisfy the compiler's all encompassing desire to make everything threadsafe, even when there isn't a single thread::spawn in the whole codebase.

## Dynamic borrow checking causes unexpected crashes after refactorings
As I'm writing this I just discovered yet another case of our game crashing because of an overlapping World::query_mut. We've been using hecs for about 2 years now, these aren't the trivial sort of "oh I accidentally nested two queries, oopsie" you run into when you first start using the library. But rather one part of the code being top level that runs a system that does something, and then an independent part of the code doing something simple with ECS somewhere deep down, and then through a large scale refactoring these end up overlapping unexpectedly.

It's not the first time I've had this happen, and the commonly suggested solution is "your code is just poorly structured, that's why you're running into these issues, you have to refactor and design it properly". Countering such arguments is relatively difficult, because at the core they're not wrong, this happens because some parts of the codebase were suboptimally designed. The problem is, it's yet another case of Rust forcing a refactoring where no other language would. Overlapping archetypes aren't necessarily a crime, and non-Rust ECS solutions like flecs are happy to allow this.

But this issue isn't limited to just ECS. We've had it happen time and time again with the use of RefCell<T>, where two .borrow_mut() end up overlapping and causing an unexpected crash.

The thing is, these aren't always just because of "bad code". People will say "borrow for the shortest amount you can" to work around the issue, but this isn't free. Obviously this again depends on having the code structured in the right way, but at this point I hope we've established that gamedev isn't server development, and code isn't always organized optimally. Sometimes one might have a loop that needs to use something from a RefCell, and it makes a lot of sense to extend the borrow over the whole loop instead of just borrowing where it's needed. This can immediately lead to an issue if the loop is large enough and calls a system that might need the same cell somewhere inside, usually with some conditional logic. Once could again argue "just use indirection and do the conditional thing through an event", but then again we're taking a tradeoff of having the gameplay logic spread over the codebase instead of just having 20 lines of obviously readable code.

In a perfect world everything would be tested on every refactoring, every branch would be evaluated, and code flow would be nicely linear and top down where these things don't ever occur. One wouldn't have to use a RefCell but would carefully design their functions so they can pass down the right context object or only the needed parameters.

Unfortunately, I don't see this being even remotely realistic for indie game development. Time spent refactoring a feature that might get removed 2 weeks down the line is time wasted, making RefCells a desirable solution to partial borrows where otherwise data would have to be re-organized into differently shaped context structs, or function parameters would have to be changed all over the place to drill down the right parameters, or indirection would have to be employed to separate things out.

## Context objects aren't flexible enough
Since Rust has a relatively unique set of constraints on programmers, it ends up creating a lot of self-inflicted issues that end up having solutions that don't necessarily come up in other languages as often.

An example of this is a context object that gets passed around. In almost every other language it's not a big problem to introduce global state, be it in the form of global variables or singletons. Rust does unfortunately make that quite a bit more difficult for all sorts of reasons mentioned above.

First solution that one would come up with is "just store the references to whatever you need for later", but anyone who has used Rust for more than a few days will recognize that this is just not going to be possible. The borrow checker will require every reference field to have its lifetime tracked, and because lifetimes become generics that poison every single usage point of the type, it's not even something that can be easily experimented with.

There's more than one problem here, but I feel the need to point this out a bit more explicitly, as it may not be obvious to those who haven't tried. On a surface level, it may seem that "what if I just use the lifetimes?", e.g.

```
struct Thing<'a>
  x: &'a i32
}
```

The problem is, what if we now want a fn foo(t: &Thing) ... well of course we can't, Thing is generic over a lifetime, so this has to become fn foo<'a>(t: &Thing<'a>) or worse. Same thing if we try to store Thing in another struct, now we end up with

```
struct Potato<'a>,
  size: f32,
  thing: Thing<'a>,
}
```

and even though Potato might not really care about Thing, being in Rust lifetimes are to be taken with utmost seriousness, and we can't just ignore them. Things are actually much worse than they seem, because lets say you do end up going down this road, and try to figure out things with lifetimes.

Rust also does not allow unused lifetimes, so say that you have

```
struct Foo<'a> {
    x: &'a i32,
}
```

but as you're refactoring your codebase you end up wanting to change this to

```
struct Foo<'a> {
    x: i32,
}
```

now that is of course completely prohibited, because you'd have an unused lifetime, and we can't have that. This may seem very minor, and in some languages this is somehow desired even in simpler cases, but the problem is lifetimes often require a decent amount of "problem solving" and "debugging" where one tries a few different things, and trying things with lifetimes often means adding or removing lifetimes, and removing lifetimes very often means "oh this is now unused, you have to remove it everywhere", leading to gigantic cascading refactorings. I have tried going down this road a few times over the years, and honestly, one of the most infuriating things is trying to iterate on a very simple change with lifetimes only to be forced to change 10 different places on every single change.

But even if the above wasn't the case, in many cases we can't just "store a reference to something" anyway, because the lifetimes wouldn't work out.

One alternative that Rust provides here is shared ownership, in the way of Rc<T> or Arc<T>. This of course works, but is heavily frowned upon. One of the things I've realized after using Rust for a while is that using these can actually save one quite a bit of sanity, although it does require not telling your Rust friends about the code you write anymore, or at least hiding it and pretending it doesn't exist.

Unfortunately, there are still many cases where shared ownership is just the bad solution, possibly for performance reasons, but sometimes you just don't have control over the ownership and can only get a reference.

The #1 trick in Rust gamedev is "if you pass in references top down every frame, all your lifetime/reference problems disappear". This actually works very well, and is similar to React's props being passed top down. There's only one issue, and that is that now you need to pass everything into every function that needs it.

At first it seems obvious and easy, just design your code correctly and you won't have any issues, lol. Or at least that's what many would say, and specifically "if you're having issues with this, your code is ugly/wrong/bad/spaghetti" or "you shouldn't be doing it that way" and you know, the usual.

Lucky for us, there is an actual solution, which is to create a context struct that is passed around and that contains all those references. This ends up having a lifetime, but only one, and ends up looking something like this:

```
struct Context<'a> {
  player: &'a mut Player,
  camera: &'a mut Camera,
  // ...
}
```

Every function in your game can then just accept a simple c: &mut Context and get what it needs. Great, right?

Well, only as long as you don't end up borrowing anything. Imagine you want to run a player system, but also hold onto the camera. The player_system just like everything in the game wants c: &mut Context, because you want to be consistent and avoid having to pass 10 different parameters into things. But when you try doing this:

```
let cam = c.camera;

player_system(c);

cam.update();
```

you'll just be met with the usual "can't borrow c because it's already borrowed", as we touched a field, and the rules of partial borrows say that if you touch a thing the whole thing is borrowed.

It doesn't matter if player_system only touches c.player, Rust doesn't care what's on the inside, it only cares about the type, and the type says it wants c, so it must get c. This may seem like a dumb example, but in larger projects with larger context objects it becomes unfortunately quite common to want some subset of the fields somewhere, while also conveniently wanting to pass the rest of the fields elsewhere.

Now lucky for us, Rust isn't completely dumb, and it would allow us to do player_system(c.player), because partial borrows allow us to borrow disjoint fields.

This is where defenders of the borrow checker will say that you simply designed your context object wrong, and that you should be splitting it up either into multiple context objects, or group your fields based on their usage so that partial borrows can be utilized. Maybe all the camera stuff is in one field, all the player stuff is in another field, and then we can just pass that field into player_system and not the whole c and everyone is happy, right?

Unfortunately, this falls under the #1 problem this article tries to address, and that is that what I want to be doing is working on my game. I'm not making games to have fun with the type system and figure out the best way to organize my struct to make the compiler happy. There is absolutely nothing I'm gaining in terms of maintainability of my single threaded code when I reorganize my context object. Having done this exact thing quite a few times I'm very much certain that the next time I have a playtest and get new suggestions for my game I'll probably have to change the design again.

The problem here is, the code is not being changed because the business logic is changing, it's being changed because the compiler isn't happy with something that is fundamentally correct. It may not follow the way the borrow checker works because it only looks at types, but it is correct in the sense that if we instead passed all the fields we're using it would compile just fine. Rust is making us make a choice between passing 7 different parameters or refactoring our struct any time something is moved around, where both of those options are annoying and wasting time.

Rust doesn't have a structural type system where we could say "a type that has these fields", or any other solution to this problem that would work without having to redefine the struct and everything that uses it. It simply forces the programmer to do the "correct" thing.

## Positives of Rust
Despite the whole article being very much against Rust, I'd like to list a few things that I found as positives, and that really helped us during development.

If it compiles it often kinda just works. This is both a meme but in some sense not really. There have been many times where I was surprised by how far one can take "compiler driven development" and actually succeed. Rust's biggest strength by far is that when one is writing code that is fitting for Rust, things go very well, and the language guides the user along the right path.

From my perspective the biggest strength here are CLI tools, data manipulation and algorithms. I've spent a non-trivial amount of time basically writing "Python scripts in Rust", where what would usually be small utilities that most would use Python or Bash for I chose to use Rust (both for learning and seeing if it'd work), and quite often I was surprised that this actually worked. I definitely would not want to be doing the same in C++.

Performance by default. As we're moving back to C#, I ended up looking a bit into Rust vs C# performance at the more granular level, trying to match specific algorithms 1:1 between the two languages and tried to get performance as close as possible. Still, Rust would come out on top by a rough order of 1:1.5-2.5 after some efforts on the C# front. This probably isn't too surprising to those who consume benchmarks on a daily basis, but having gone through this myself and having really tried, I was very pleasantly surprised how naturally occurring Rust code would just be really fast.

I do want to point out that Unity's Burst compiler improves on C#'s performance quite a bit, but I don't have enough A/B data to provide specific numbers, and have only observed significant speedups on C# alone.

That being said, in all the years of Rust I've been continually pleasantly surprised how well the code runs, even despite doing very stupid things, which I often like to do. I will note that this is all predicated on having the following in Cargo.toml

```
[profile.dev]
opt-level = 1
[profile.dev.package."*"]
opt-level = 1
```

As I've seen many many many people asking about things being slow, only to find they're just making a debug build. And just as Rust is very fast with optimizations turned on, it is very slow with optimizations turned off. I use opt-level = 1 instead of 3 because in my testing I haven't noticed a difference in speed, but 3 compiled a bit slower, at least on the code I tested on.

Enums are really nicely implemented. Everyone using Rust probably knows this, and I will say that as time passes I tend to move to more dynamic structuring of things rather than strictly with enums and pattern matching, but at least for the cases where enums fit, they end up being very nice to work with, and probably my favorite implementation across the languages I've used.

Rust analyzer. I wasn't sure if I should put this in the positives or negatives. I'm putting it in the positives because I would 100% not be able to write Rust without it anymore. Having first started with Rust in 2013 or so, the tooling around the language has improved dramatically, to the point where it's actually very very useful.

The reason I considered putting it in the negatives is that it is still one of the more broken language servers I have used. I understand that this is because Rust is a complicated language, and having spoken to a lot of people about this, I think my projects might be just a bit cursed (it's probably my fault) because it tends to crash and not work for me all the time (yes I have updated, it's been happening for over a year across machines/projects). But despite all that, it's still incredibly useful and helpful, and significantly helps with writing Rust.

Traits. While I'm not fully a fan of banishing inheritance completely, I do think the trait system is quite nice, and fits Rust very well. If only we could get some relaxation on the orphan rule things would be really great. Still, being able to use extension traits is one of my favorite things in the language.

## Closing thoughts
We've been using Rust on basically all of our games since mid 2021. This was when BITGUN initially started as a Godot/GDScript only project, and as we ran into issues with Godot's pathfinding (both performance and functionality wise) I looked into alternatives, found gdnative, and then was recommended godot-rust. It wasn't the first time I've seen or used Rust, but it was the first serious usage for game development, being preceeded only by game-jam-y projects.

Since then Rust was the langauge I used for everything. I got excited about things like building my own renderer/framework/engine, and early versions of Comfy were born. Many things followed, from small game jam prototypes of CPU raytracing, playing around with simple 2D IK, trying to write a physics engine, implementing behavior trees, implementing a single threaded coroutine focused async executor, to building simulation-y NANOVOID, and finally Unrelaxing Quacks, our first and last Comfy game to be released, which actually goes out today on Steam, at the same time as this article.

This article is in large part inspired by our struggles while working on NANOVOID and Unrelaxing Quacks, as these projects were unburdened by the lack of knowledge of Rust that we had while working on BITGUN initially. These projects also had the benefit of trying most of the Rust gamedev ecosystem on more than one occasion. We tried Bevy at more than one occasion, with BITGUN being the first game we tried to port, and Unrelaxing Quacks being the last. During the two years of developing what would become Comfy the renderer was rewritten from OpenGL to wgpu to OpenGL to wgpu again. At the time of writing this, I've been programming for roughly 20 years, starting with C++, and moving through all sorts of languages, spanning PHP, Java, Ruby, JavaScript, Haskell, Python, Go, C#, and having released a game on Steam in Unity, Unreal Engine 4, and Godot. I'm the type of person who likes to experiment and try all sorts of approaches just to make sure I'm not missing something. Our games may not be the greatest by most measures, but we have thoroughly explored the options available with the hope of finding the most preferrable solution.

I'm saying all of this to dispel any ideas that there wasn't enough effort put into trying Rust, and that the article isn't written from a point of ignorance or not trying the correct approach. The #1 argument I hear people say when someone points out issues with Rust as a language is, jokingly, "you just don't have enough experience to appreciate this". We repeatedly tried both more dynamic and fully static approaches to doing things. We tried pure ECS, and we also tried no ECS.

For those reading this who are concerned about the future of Comfy, here's my thoughts on it.

Comfy has been largely "finished" from the perspective of 2D games. This should be evidenced that we're releasing a full game in it, and I want to clarify that our game runs against the master branch. If your goals are to build something of similar complexity and quality, it should be pretty obvious that you can.

That being said, there are still things that are desirable that Comfy does not currently provide, namely improvements on custom shaders and post processing passes. There's also the question of "maintenance future" since I won't be working on any more Rust games.

Those that have been active on our Discord already know that the plan is to port Comfy's renderer to Macroquad, which means completely removing all traces of wgpu and winit, and instead using Macroquad for windowing, input, and rendering. There's a few reasons and benefits we get from this:

* A large portion of the codebase can be just deleted, along with many weird edge cases. Users don't gain anything from "having a custom wgpu renderer", what matters is functionality, and that won't change.
Shaders and post processing become a lot more flexible, just by the fact that Macroquad already has everything in place.
* More platforms supported, as Macroquad/Miniquad has the widest reach in the ecosystem, and Comfy currently runs only on things that wgpu runs on.
* Stable future, where Comfy can become a high level convenience layer on top of something that is maintained by an existing community where people know what they're doing.

Some maybe asking why this wasn't done in the first place. Initially, some of our Comfy projects were written on top of Macroquad, but at some point I wanted to have HDR and Bloom, which Macroquad did not support. Comfy was created by copy-pasting Macroquad's APIs and extending them with z-indexing and y-sorting, and re-implementing the whole renderer underneath.

But as of recently, Miniquad/Macroquad now does support f16 textures, which means we can get all of that without needing a custom renderer. There's already an ongoing effort to port things, but has been largely stale due to our attempt to release Unrelaxing Quacks in a timely manner. I do however plan to resume working on this after the release, and considering basic things already work there, I'm hopeful the port shouldn't end up being too complicated.

---

Lastly, I will do a shameless plug for our latest game, because after almost a year of working on it it would be stupid not to :)

Unrelaxing Quacks is a survivors game, but fast. It's a game that puts you straight into action and doesn't waste your time. Thanks to Rust we managed to have lots of enemies and projectiles while still achieving great performance.

We've also put a lot of effort into polishing the absolute core mechanics such as movement and shooting, to make sure everything felt good.


If you liked the article and would want to support us, buy the game and review it on Steam. It doesn't have to be positive, be honest about how you felt when playing! Reviews greatly help the developer as Steam will increase the game's visibility once it reaches 10 reviews, regardless of whether they're positive or negative.


If you'd like to comment on the article, the respective discussion is here on /r/programming, /r/rust, Hacker News, and Twitter.

The article is also available in Chinese.

If anyone wants to translate the article or use it in any way, feel free to do so without needing to ask, just please provide a link back to the original article somewhere in the text.