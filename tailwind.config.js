/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                // Meal category colors
                'meal-breakfast': '#FCD34D',
                'meal-lunch': '#7DD3FC',
                'meal-dinner': '#C4B5FD',
                'meal-snack': '#86EFAC',
            },
        },
    },
    plugins: [],
}
