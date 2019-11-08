function sendQuote(url, data){
    fetch(url, {
        method: 'POST', // or 'PUT'
        body: JSON.stringify(data), // data can be `string` or {object}!
        headers:{
          'Content-Type': 'application/json'
        }
    });
}

// Without jQuery
// Define a convenience method and use it
var ready = (callback) => {
    if (document.readyState != "loading") callback();
    else document.addEventListener("DOMContentLoaded", callback);
}

ready(() => { 
    
    document.querySelector('#send').addEventListener('click', (e) => {
        console.log('Sending POST');
        var data = {
            text: document.querySelector("#input-text").value,
            author: document.querySelector("#input-author").value
        };

        // check if any text is empty
        if(data.text === '' || data.author === ''){
            alert('Text or author is empty');
            return;
        }

        fetch('/newquote', {
            method: 'POST', // or 'PUT'
            body: JSON.stringify(data), // data can be `string` or {object}!
            headers:{
                'Content-Type': 'application/json'
            }
        });

        alert("Quote subitted!!");
        
        document.querySelector("#input-text").value = '';
        document.querySelector("#input-author").value = '';
    })

});
