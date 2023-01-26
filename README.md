# AutoSDF

procedurally generated signed distance functions

![a screenshot from AutoSDF](example3.png)

**this is a very GPU-intensive application. if it's running slow, be sure to reduce `scale`, `sdf-length` and/or `color-length` in `settings.txt`.**
On the other hand, if you have a good GPU:
- be sure to increase `max-iterations` to something like 100-200. It will look much nicer.
- I also recommend increasing `sdf-length` to get more interesting shapes.

## controls

- move mouse to look around
- W,A,S,D/arrow keys to move forward/backwards/left/right + Q,E/PageUp,PageDown to move up/down
- R to create a new SDF - use this a lot! most SDFs are boring!
- Space to "unpause time" (start animating).
  Press space again to pause time, and shift+space to rewind time.
- [ and ] to go forwards and backwards in time
- 0 to reset location + time
- =/- (equals/minus) to expand/contract the surface (change the "level set" of the SDF you're looking at)
- Ctrl+C to copy SDF to clipboard, Ctrl+V to paste SDF from clipboard. On Linux
  if you close the application the clipboard contents will be lost (thanks a lot X11).
- F to go fullscreen
- F10 to take a screenshot (saved to a folder called `screenshots`). The SDF string
  is saved in the PNG file's metadata. If you open it with notepad, you'll see it towards the beginning of the file.
- Press Ctrl+Q to quit (or press escape and click the "quit" button)
- You can use Shift to move faster and Ctrl to move slower.

## saving SDFs

if you get an SDF which looks cool, you can copy it with Ctrl+C, and save it somewhere/share it with
your friends.

also a list of all SDFs you've generated is stored in `scenes.txt`.

## cool SDFs

here are some cool SDFs i've found. take a look at them (using Ctrl+V) and especially try using minus/equals!
```
a263736466a167436f6d706f736583a1695472616e736c61746583a163463332fa3f4811e4a163463332fa3f78c51ea16454696d6582fa3d90de6dfa3f596b98a1634d697883a167436f6d706f736583a167436f6d706f736582684964656e74697479684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3eef3a44684964656e74697479684964656e74697479a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3f6ad464a165546f727573a266726164697573a163463332fa3fcd633269746869636b6e657373a163463332fa3d887747684964656e74697479a16454696d6582fabcad7e74fa3f444f5c684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a167436f6d706f736582a167436f6d706f736582a166526f7461746583a163463332fa3f139110a163463332fa3e8c6384a163463332fa3edc7998675369676d6f6964a16353696ea16454696d6582fa3ce10f8cfa3e42f2c8a167436f6d706f736582a16f496e66696e6974654d6972726f7273a163463332fa3f4e41c8a167436f6d706f736582a16641726374616ea16454696d6582fa3dc7acedfa3f76784ca16353696ea163463332fa3f62c45a
a263736466a167436f6d706f736583a166526f7461746583a163463332fa3f76cab2a163463332fa3d81cad0a163463332fa3f76ebd4a1634d696e82a167436f6d706f736583675369676d6f6964a1634d696e82a1634d697883a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3e5a3e68684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f46ade4684964656e74697479a163463332fa3f551da4a1634d696e82a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f306be8684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3ca99ac0684964656e74697479684964656e74697479a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3e9febeca167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3ee05424a167436f6d706f736583675369676d6f6964a1634d696e82a166537068657265a163463332fa3e16dcf0a166537068657265a163463332fa3f48f0dc684964656e74697479684964656e74697479684964656e74697479684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a16353696ea163463332fa3f1c2a8e675369676d6f6964
a263736466a1634d696e82a1634d696e82a166537068657265a163463332fa3f2365aca1634d697883a167436f6d706f736583a16641726374616ea163463332fa3eb04a5ca167436f6d706f736583675369676d6f6964a169536d6f6f74684d696e82a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3eeb25b8684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f10d6a2684964656e74697479684964656e74697479684964656e74697479a165546f727573a266726164697573a163463332fa3d9099f069746869636b6e657373a163463332fa3e00b102a163463332fa3f06b5a2a1634d696e82a165546f727573a266726164697573a163463332fa40121b0169746869636b6e657373a163463332fa3e32f4faa1634d697883a16443756265a163463332fa3f7f9dc8a1634d697883a165546f727573a266726164697573a163463332fa3f9286e369746869636b6e657373a163463332fa3d8a3f27a167436f6d706f736583a167436f6d706f736582a167436f6d706f736582684964656e74697479684964656e74697479a16f496e66696e6974654d6972726f7273a163463332fa3ea62688a168426f784672616d65a26473697a65a163463332fa3fc0fb4969746869636b6e657373a163463332fa3e472462684964656e74697479a163463332fa3f69a73ea163463332fa3f5b9c9e6e636f6c6f725f66756e6374696f6ea16f496e66696e6974654d6972726f7273a163463332fa3ef4ea8c
a263736466a169536d6f6f74684d696e82a1634d696e82a167436f6d706f736583a167436f6d706f736582a16641726374616ea163463332fa3e6d7230a1695472616e736c61746583a163463332fa3e7262f8a163463332fa3eece0eca163463332fa3f49c42ca168426f784672616d65a26473697a65a163463332fa3ff3ee1169746869636b6e657373a163463332fa3df6dfed684964656e74697479a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3f2a2de8a165546f727573a266726164697573a163463332fa3fc93f1e69746869636b6e657373a163463332fa3e0fa700684964656e74697479a169536d6f6f74684d696e82a169536d6f6f74684d696e82a1634d696e82a169536d6f6f74684d696e82a167436f6d706f736583675369676d6f6964a1634d696e82a166537068657265a163463332fa3e0586b0a166537068657265a163463332fa3f4d6214684964656e74697479a167436f6d706f736583a167436f6d706f736582684964656e74697479684964656e74697479a1634d697883a166537068657265a163463332fa3f5c46e6a166537068657265a163463332fa3f3f4896a163463332fa3f10ba30684964656e74697479a166537068657265a163463332fa3ef3b604a167436f6d706f736583a165537153696ea163463332fa3ef1bfb8a1634d697883a1634d697883a168426f784672616d65a26473697a65a163463332fa3ee4801069746869636b6e657373a163463332fa3dc84d37a168426f784672616d65a26473697a65a163463332fa4019d5d269746869636b6e657373a163463332fa3d5d0307a163463332fa3eb7554ca167436f6d706f736583a165537153696ea163463332fa3f31c33ca167436f6d706f736583684964656e74697479a166537068657265a163463332fa3e7db3c0684964656e74697479684964656e74697479a163463332fa3f3176c0684964656e74697479a165546f727573a266726164697573a163463332fa3f253f0c69746869636b6e657373a163463332fa3c96c48d6e636f6c6f725f66756e6374696f6ea16f496e66696e6974654d6972726f7273a163463332fa3ea219f4
a263736466a1634d696e82a1634d697883a1634d697883a16c564c696e655365676d656e74a16454696d6582fa3c8b2f18fa3f3678a6a168547269507269736d82a16454696d6582fabdb2b99afa3f656318a16454696d6582fabd37e33afa3f1b42c6a16454696d6582fabd3dfe1afa3e32b6b0a167436f6d706f736583a16f496e66696e6974654d6972726f7273a16454696d6582fa3d9f9eb1fa3ddac4e0a168547269507269736d82a16454696d6582fa3c1192e8fa3eca5188a16454696d6582fa3d2c25dafa3e9d72f4684964656e74697479a16454696d6582fa3d837b71fa3e73be30a167436f6d706f73658366576962626c79a167436f6d706f73658366576962626c79a16843796c696e64657282a16454696d6582fabd96901dfa3ed15cfca16454696d6582fa3d7cd81afa3dadb7d0684964656e74697479684964656e746974796e636f6c6f725f66756e6374696f6ea1634d697883a1634d697883a167436f6d706f736582a166526f7461746583a16454696d6582fa3db2bf97fa3ef7ee74a16454696d6582fa3c29fab0fa3e31fb78a16454696d6582fabc315b80fa3f0f75baa16353696ea16454696d6582fabc3cdc68fa3f2c1a1aa16353696ea16454696d6582fa3d947937fa3efcb818a16454696d6582fa3d8cd239fa3d6083e066576962626c79a16454696d6582fabd734040fa3f4e39e8
a263736466a16653696e436f7382a16c564c696e655365676d656e74a16454696d6582fabd6f12dafa3f601600a167436f6d706f736583a167436f6d706f736582a1695472616e736c61746583a16454696d6582fabd1eb54dfa3e66a640a16454696d6582fabd8a51a4fa3f25632aa16454696d6582fabbadc0a0fa3f0f7794a1695472616e736c61746583a16454696d6582fa3da911fdfa3ea7fdd4a16454696d6582fa3d04bdf2fa3f6cae42a16454696d6582fa3ce2d374fa3f3716f6a1634d696e82a168547269507269736d82a16454696d6582fa3dab2981fa3f3b4b96a16454696d6582fabd7c9badfa3f7558faa165546f727573a266726164697573a16454696d6582fabdaad43efa3e84f9e469746869636b6e657373a16454696d6582fabc61bdcdfa3c4d3880684964656e746974796e636f6c6f725f66756e6374696f6ea1634d697883a166526f7461746583a16454696d6582fabdc72393fa3e34f9d8a16454696d6582fabc813218fa3f675278a16454696d6582fa3ca4e728fa3f59b236a167436f6d706f736582a16353696ea16454696d6582fabdbce95dfa3f374964a16f496e66696e6974654d6972726f7273a16454696d6582fa3c50ad30fa3e06c2e8a16454696d6582fabd549d60fa3f2584a8
a263736466a16653696e436f7382a167436f6d706f736583a165537153696ea16454696d6582fa3be21a30fa3ef85110a169536d6f6f74684d696e82a16443756265a16454696d6582fabcbd91a8fa3ec68354a166537068657265a16454696d6582fabd8d45cafa3f45f8c8684964656e74697479a1634d696e82a169536d6f6f74684d696e82a166537068657265a16454696d6582fa3b6a4c00fa3f476626a168426f784672616d65a26473697a65a16454696d6582fa3e458346fa4007694a69746869636b6e657373a16454696d6582fa3c0dac9bfa3daf70a7a1634d697883a168426f784672616d65a26473697a65a16454696d6582fabd26ef4efa4017a0e269746869636b6e657373a16454696d6582fabc003f71fa3e4706d0a16443756265a16454696d6582fa3d8330c3fa3f43448aa16454696d6582fabd565d0dfa3f3b8da66e636f6c6f725f66756e6374696f6ea167436f6d706f736582a16641726374616ea16454696d6582fa3d17bdc2fa3f09ec7ca167436f6d706f736582a166526f7461746583a16454696d6582fabce58528fa3e8bdbf0a16454696d6582fa3d06c2a6fa3f05c282a16454696d6582fa3d13eaaefa3e411148a166526f7461746583a16454696d6582fa3cc0bdd8fa3efc278ca16454696d6582fa3cf95bc4fa3f7868faa16454696d6582fa3d3f2342fa3f585f32
a263736466a16653696e436f7382a1634d697883a167436f6d706f736583a16353696ea16454696d6582fabd9ebb46fa3f18eb88a165546f727573a266726164697573a16454696d6582fa3e6d0eb4fa402fbf0669746869636b6e657373a16454696d6582fa3c752fc2fa3e4cc7e7684964656e74697479a167436f6d706f736583a1695472616e736c61746583a16454696d6582fa3db691c1fa3ef9dc24a16454696d6582fa3d903c57fa3f6b36caa16454696d6582fa3d21edbafa3f3406246850726f6a6563745a684964656e74697479a16454696d6582fabb5f1200fa3f0cd1f4a167436f6d706f736583a165537153696ea16454696d6582fabd2f45a7fa3f0a677aa167436f6d706f736583675369676d6f6964a16c564c696e655365676d656e74a16454696d6582fabc772098fa3ed113c8684964656e74697479684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a1634d697883a1695472616e736c61746583a16454696d6582fabd066e06fa3f05798aa16454696d6582fa3dba9733fa3f0c084aa16454696d6582fabdc95d37fa3e81ed3866576962626c79a16454696d6582fabc5ec830fa3f202cb2a166526f7461746583a16454696d6582fabd0966f4fa3e27ba20a16454696d6582fabd94f970fa3e2f19b8a16454696d6582fabd588d27fa3e633d18
a263736466a167436f6d706f736583a167436f6d706f736582a167436f6d706f736582a1634d697883a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fabc62a468fa3eb47d24a1695472616e736c61746583a16454696d6582fabd6f31f3fa3dc1dfd0a16454696d6582fa3d3409a2fa3f789600a16454696d6582fabda414fafa3ead4f00a16454696d6582fabd856996fa3f0ca420a166526f7461746583a16454696d6582fa3d0206a6fa3f6ab128a16454696d6582fabdb1764afa3f37e130a16454696d6582fa3d080572fa3f16642ca1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fabd928746fa3e1b9850a167436f6d706f736583a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fabd487d53fa3f4f3e7aa167436f6d706f736583a16c506572436f6d706f6e656e7483a1644e546f4e66576962626c79684964656e74697479684964656e74697479a16653696e436f7382a166537068657265a16454696d6582fa3d9c80a1fa3ebba6c46850726f6a65637458684964656e74697479a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fa3d90c4b3fa3f73b616a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fa3d276692fa3e6b2ba06e636f6c6f725f66756e6374696f6ea167436f6d706f736582a16c506572436f6d706f6e656e7483684964656e74697479a1644e546f4e675369676d6f6964a1644e546f4ea16453717274a16454696d6582fa3c231730fa3f69f97ca16c506572436f6d706f6e656e7483684964656e74697479a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fabd0f078cfa3f2ce428a1644e546f4ea16641726374616ea16454696d6582fa3dc9389dfa3e1bea30
a263736466a1634d696e82a1634d696e82a169536d6f6f74684d696e82a167436f6d706f736583a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fa3cd10fa4fa3e7a7ee0a166537068657265a16454696d6582fa3d3a8352fa3f10627ea1644e546f4e66576962626c79a1634d6978836850726f6a656374586850726f6a6563745aa16454696d6582fa3ca53034fa3ed7ec00a169536d6f6f74684d696e82a1634d696e826850726f6a65637458a16c564c696e655365676d656e74a16454696d6582fabdb2bbeafa3d3a98a0a167436f6d706f736583a1644e546f4ea16353696ea16454696d6582fa3d2bf786fa3e6f8800a166537068657265a16454696d6582fa3d1f17fafa3edaed58684964656e74697479a169536d6f6f74684d696e82a166537068657265a16454696d6582fabcb6f0d8fa3e14d9a8a16653696e436f7382a1634d697883a168547269507269736d82a16454696d6582fabb07e060fa3e89038ca16454696d6582fa3da7736dfa3e76b8006850726f6a65637459a16454696d6582fa3d5c6dfafa3f69e29aa167436f6d706f736583a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fa3dc9b8f9fa3e7f5ca8a168426f784672616d65a26473697a65a16454696d6582fa3ddf9047fa401c9a4069746869636b6e657373a16454696d6582fabc95e1cffa3dcb43b3684964656e746974796e636f6c6f725f66756e6374696f6ea1634d697883a167436f6d706f736582a1644e546f4e675369676d6f6964a1644e546f4e675369676d6f6964a167436f6d706f736582a1644e546f4ea16453717274a16454696d6582fa3d9753c9fa3f2c7994a1644e546f4ea16f496e66696e6974654d6972726f7273a16454696d6582fabda5722dfa3f665366a16454696d6582fa3d87d6bdfa3f52c068
a263736466a169536d6f6f74684d696e82a169536d6f6f74684d696e82a167436f6d706f736583a1634d697883a1644e546f4ea165537153696ea16454696d6582fa3d03783afa3f66c584a1644e546f4e66576962626c79a16454696d6582fa3d35b64efa3e8d3e54a16653696e436f7382a168547269507269736d82a16454696d6582fabcb3b380fa3ded0d30a16454696d6582fabd473693fa3f62abcea16c564c696e655365676d656e74a16454696d6582fa3d6f5f4efa3e554e88684964656e746974796850726f6a6563745aa16653696e436f7382a16653696e436f7382a1634d6978836850726f6a6563745aa168547269507269736d82a16454696d6582fabd047d06fa3d7c4480a16454696d6582fa3d075c2efa3f00dafaa16454696d6582fabc130be8fa3e6719f8a16653696e436f73826850726f6a65637458a16843796c696e64657282a16454696d6582fa3da6e611fa3f32bf2ea16454696d6582fabcc34b4cfa3f6442a2a167436f6d706f736583a16c506572436f6d706f6e656e7483684964656e74697479684964656e74697479a1644e546f4ea16353696ea16454696d6582fa3d6ba5aefa3ebd1940a1634d697883a168547269507269736d82a16454696d6582fabd7237cdfa3e2b8f98a16454696d6582fa3c897b98fa3f325384a166537068657265a16454696d6582fabc273750fa3e589c48a16454696d6582fabdb6fa17fa3f6faf82684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a167436f6d706f736582a1634d697883a1644e546f4ea165537153696ea16454696d6582fabd611013fa3ef150b8a1644e546f4ea16353696ea16454696d6582fabd782740fa3e7c98a8a16454696d6582fabd240767fa3f091296a1644e546f4e66576962626c79a1644e546f4ea16353696ea16454696d6582fa3d9b1683fa3f048970
```

## building from source
[Install rust](https://www.rust-lang.org/tools/install) and run `cargo run --release` from this directory.
