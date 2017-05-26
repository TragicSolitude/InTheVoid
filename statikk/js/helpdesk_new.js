{
    let input = document.querySelector('#issue_image');
    let supported_file_types = [
        "image/gif",
        "image/png",
        "image/jpeg",
        "image/bmp",
        "image/webp"
    ];

    input.addEventListener('change', e => {
        var file = e.target.files[0],
            reader = new FileReader();
        
        if (supported_file_types.indexOf(file.type) === -1) {
            notify('danger', 'Please upload only images');
            document.querySelector('#iu-display').style.backgroundImage = 'none';
            e.target.value = '';
        } else {
            reader.addEventListener('loadend', () => {
                document.querySelector('#iu-display').style.backgroundImage = `url(${reader.result})`;
            });

            if (file) {
                reader.readAsDataURL(file);
            }
        }
        
    });
}