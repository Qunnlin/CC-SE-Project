import requests
import json

# Open the file with dishes
with open('query.txt', 'r') as f:
    dishes = f.readlines()

# Open the file to write responses
with open('response.txt', 'w') as r:
    # Iterate over dishes
    for dish in dishes:
        dish = dish.strip()
        print(f"Dish: {dish}")

        # Make the POST request
        post_data = {"name": dish}
        post_response = requests.post(url="http://localhost:8000/dishes", json=post_data,  headers={"Content-Type": "application/json"})
        print(f"Post Response: {post_response.text}")

        # Make the GET request
        get_response = requests.get(url=f"http://localhost:8000/dishes/{dish}",  headers={"Content-Type": "application/json"})
        print(f"Response: {get_response.text}")

        # Parse the response
        response_json = get_response.json()
        calories = response_json.get('cal')
        sodium = response_json.get('sodium')
        sugar = response_json.get('sugar')

        # Write to the response file
        r.write(f"{dish} contains {calories} calories, {sodium} mgs of sodium, and {sugar} grams of sugar\n")

