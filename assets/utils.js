function toggleHide(id) { 
    const flip = v => v === 'inline' ? 'none' : 'inline';
    document.getElementById(id).style.display = flip(document.getElementById(id).style.display);
}