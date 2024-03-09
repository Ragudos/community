(function(){
    const btn = document.getElementById('scroll-up');

    function scrollToTop() {
        window.scrollTo({top: 0, behavior: 'smooth'});
    }

    if (btn) {
        btn.addEventListener('click', scrollToTop);
    }
})();
