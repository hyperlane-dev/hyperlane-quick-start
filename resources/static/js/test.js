// Test JavaScript file for static file routing
console.log('Static file routing test');

document.addEventListener('DOMContentLoaded', function() {
    console.log('DOM loaded');
    
    const testElement = document.getElementById('test');
    if (testElement) {
        testElement.textContent = 'Static file routing is working!';
    }
});

function testFunction() {
    return 'Hello from static JS file!';
}