import hasher from "../hasher/Cargo.toml";

var input = document.getElementById( "input" );
var output = document.getElementById( "output" );
output.innerText = hasher.sha1( input.value );

input.addEventListener( "keyup", function( event ) {
    output.innerText = hasher.sha1( input.value );
});
