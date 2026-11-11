# Decision 6: What does a message hold?

In my design recommendation, a message holds a role and a list of parts, and each part is a value of an enum in the style of:

```rust
#[non_exhaustive]
pub enum MultimodalInput {
    Text( … ),
    Image {  …  },
    Audio {  …   },
    Video {  …   },
}
```

I went with this design because it contains several advantages: first, the shape of the object is checked at compile time rather than at run time, and secondly, the reader can easily and directly learn what each of the variants entail from the code itself. In case we add later modalities to the fixed list, over which a user is performing a match, the enum object must have the attribute #[non_exhaustive] in order to require the users to implement a catch-all which would allow for the enum’s extensibility without compiler failure. However, a conversion from the Engine’s JSON content is needed, due to the Engine’s continuous production of JSON content from Lua, which must be transformed into the new enum (whose design depends on Decision #2), while enforcing that if, during the conversion, an unknown (unlisted in enum) part is encountered, it returns an error and does not silently drop it.

The reason why I chose this design over the current raw JSON approach is that message_from_validated_parts stores the content unchecked and relies on the Engine’s Lua layer, which a standalone crate outside of Runvil will not have.

I also considered tensors but those are optimized for ML pipelines on local models rather than cloud providers. Finally, I also considered trait objects which would allow for custom user-created variants to be added as long as they implement the required traits, but I believe that it is better for the types to be defined in the enum, due to the fact that a trait object would no longer enforce that shapes are supported at compile time. Thus to take advantage of Rust’s safety features and compiled nature, I would recommend an enumerated approach.
