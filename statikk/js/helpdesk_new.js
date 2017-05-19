{
    let input = document.querySelector('#issue_image');

    input.addEventListener('change', e => {
        var file = e.target.files[0],
            reader = new FileReader();
        
        
        
        reader.addEventListener('loadend', () => {
            document.querySelector('#iu-display').style.backgroundImage = `url(${reader.result})`;
        });

        if (file) {
            reader.readAsDataURL(file);
        }
    });
}