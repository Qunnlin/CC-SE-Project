# Python File that populates the database with test data and runs test queries

import requests

# URL for the API
MEALS_BASE = "http://localhost:5001/"
DIETS_BASE = "http://localhost:5002/"

# Endpoint for the API
MEALS_ENDPOINT = "meals"
DIETS_ENDPOINT = "diets"
DISHES_ENDPOINT = "dishes"

# Test data
DISHES = [
    {
        "name": "100g Chicken Breast",
    },
    {
        "name": "100g Salmon",
    },
    {
        "name": "100g Beef",
    },
    {
        "name": "100g Pork",
    },
    {
        "name": "100g Tofu",
    },
    {
        "name": "200g Broccoli",
    },
    {
        "name": "200g Carrots",
    },
    {
        "name": "200g Spinach",
    },
    {
        "name": "150g Rice",
    },
    {
        "name": "150g Pasta",
    },
    {
        "name": "150g Potatoes",
    },
    {
        "name": "150g Vanilla Ice Cream",
    },
    {
        "name": "150g Chocolate Ice Cream",
    },
    {
        "name": "150g Strawberry Ice Cream",
    },
]

MEALS = [
    {
        "name": "Breakfast",
        "appetizer": 1,
        "main": 2,
        "dessert": 3,
    },
    {
        "name": "Lunch",
        "appetizer": 4,
        "main": 5,
        "dessert": 6,
    },
    {
        "name": "Dinner",
        "appetizer": 7,
        "main": 8,
        "dessert": 9,
    },
    {
        "name": "Snack",
        "appetizer": 12,
        "main": 13,
        "dessert": 14,
    },
]

DIETS = [
    {
        "name": "Keto",
        "cal": 2000,
        "sodium": 2000,
        "sugar": 2000,
    },
    {
        "name": "Paleo",
        "cal": 1000,
        "sodium": 500,
        "sugar": 200,
    },
    {
        "name": "Vegan",
        "cal": 1500,
        "sodium": 1000,
        "sugar": 1000,
    },
    {
        "name": "Sugar Free",
        "cal": 2000,
        "sodium": 2000,
        "sugar": 0,
    },
    {
        "name": "Low Sodium",
        "cal": 2000,
        "sodium": 100,
        "sugar": 2000,
    },
]

# Populate the database with test data
def populate():
    # Populate dishes
    for dish in DISHES:
        print(requests.post(MEALS_BASE + DISHES_ENDPOINT, json=dish))

    # Populate meals
    for meal in MEALS:
        print(requests.post(MEALS_BASE + MEALS_ENDPOINT, json=meal))

    # Populate diets
    for diet in DIETS:
        print(requests.post(DIETS_BASE + DIETS_ENDPOINT, json=diet))

# Run test queries
def test():
    # Test meals
    print("Testing GET meals...")
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "/1"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "/2"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "/Breakfast"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "/Lunch"))

    # Test diets
    print("Testing GET diets...")
    print(requests.get(DIETS_BASE + DIETS_ENDPOINT))
    print(requests.get(DIETS_BASE + DIETS_ENDPOINT + "/1"))
    print(requests.get(DIETS_BASE + DIETS_ENDPOINT + "/2"))

    # Test dishes
    print("Testing GET dishes...")
    print(requests.get(MEALS_BASE + DISHES_ENDPOINT))
    print(requests.get(MEALS_BASE + DISHES_ENDPOINT + "/1"))
    print(requests.get(MEALS_BASE + DISHES_ENDPOINT + "/2"))
    print(requests.get(MEALS_BASE + DISHES_ENDPOINT + "/100g Chicken Breast"))
    print(requests.get(MEALS_BASE + DISHES_ENDPOINT + "/100g Salmon"))

    # Test meals with diets
    print("Testing GET meals with diets...")
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "?diet=Keto"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "?diet=Paleo"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "?diet=Vegan"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "?diet=Sugar Free"))
    print(requests.get(MEALS_BASE + MEALS_ENDPOINT + "?diet=Low Sodium"))


if __name__ == "__main__":
    populate()
    test()


