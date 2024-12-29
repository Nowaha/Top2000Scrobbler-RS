# Top 2000 Scrobbler
Rust program that allows you to scrobble songs from the Dutch [NPO Radio 2 Top 2000](https://www.nporadio2.nl/top2000) as they are being played, and show it as your Discord status.

### Environment Variables
To be able to run the program, you need to have set some environment variables.

You need to have a Last.fm application. You can create one [here](https://www.last.fm/api/account/create). Just enter a name, the rest can be left empty. After, you can set the following environment variables:
- `LASTFM_API_KEY`: The API key of your Last.fm application.
- `LASTFM_API_SECRET`: The secret of your Last.fm application.

There are also 2 optional variables to speed up the login. If you don't set these, you will be prompted to enter your Last.fm username and password.
- `LASTFM_USERNAME`: Your Last.fm username.
- `LASTFM_PASSWORD`: Your Last.fm password.

The application supports the creation of a `.env` file containing these values.

### Discord SDK
#### To run
To run, you need to have a single file from the Discord SDK. These are included in [the releases](https://github.com/Nowaha/Top2000Scrobbler-RS/releases), but can be found [here](https://discord.com/developers/docs/developer-tools/game-sdk) for a manual download. Windows needs `v2.5.6`, but Linux and MacOS work with either `v2.5.6` or `v3.x+`.

Depending on your OS, take the following file:
- Windows: `discord_game_sdk/lib/x86_64/discord_game_sdk.dll`
- Linux: `discord_game_sdk/lib/x86_64/libdiscord_game_sdk.so` (might need to rename and prepend 'lib')
- MacOS: `discord_game_sdk/lib/x86_64/libdiscord_game_sdk.dylib` (might need to rename and prepend 'lib')

And place it in the same directory as the executable to run.
#### To build
You need to have the Discord SDK downloaded and need to have 2 values set to build the project.

**In short:**
1. Download the Discord Game SDK from [here](https://discord.com/developers/docs/game-sdk/sdk-starter-guide#downloading-the-sdk).
   - Windows needs `v2.5.6`
   - Linux/MacOS work with either `v2.5.6` or `v3.x`
2. Extract it.
3. Set the `DISCORD_GAME_SDK_PATH` environment variable to the path of the extracted SDK.
4. Depending on your OS, run / set in your environment:
    ```shell
    # Linux: prepend with `lib` and add to library search path
    cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.so
    export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+${LD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64
    
    # Mac OS: prepend with `lib` and add to library search path
    cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.dylib
    export DYLD_LIBRARY_PATH=${DYLD_LIBRARY_PATH:+${DYLD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64
    
    # Windows: change `dll.lib` to `lib` (won't affect library searching)
    cp $DISCORD_GAME_SDK_PATH/lib/x86_64/discord_game_sdk.{dll.lib,lib}
    cp $DISCORD_GAME_SDK_PATH/lib/x86/discord_game_sdk.{dll.lib,lib}
    ```
   
For more information, see the [documentation of crate 'discord_game_sdk'](https://docs.rs/discord_game_sdk/latest/discord_game_sdk/#usage) (see: `Usage` & `link`)