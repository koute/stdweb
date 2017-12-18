"use strict";

const hasher = require( './target/wasm32-unknown-unknown/release/hasher.js' );

const string = "fiddlesticks";
const hash = hasher.sha1( string );

console.log( "Hash of " + string + " is '" + hash + "'" );
