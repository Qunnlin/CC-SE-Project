import requests
import json

URL = "http://localhost:8000"


def http_get(resource: str):
    response = requests.get(url=f"{URL}/{resource}", headers={"Content-Type": "application/json"})
    return response


def http_delete(resource: str):
    response = requests.delete(url=f"{URL}/{resource}", headers={"Content-Type": "application/json"})
    return response


def http_post(resource: str, data: {}):
    response = requests.post(url=f"{URL}/{resource}", headers={"Content-Type": "application/json"},
                             data=json.dumps(data))
    return response


def http_put(resource: str, data: {}):
    response = requests.put(url=f"{URL}/{resource}", headers={"Content-Type": "application/json"},
                            data=json.dumps(data))
    return response


def post_raw(resource: str, data: {}, headers: {}):
    response = requests.post(url=f"{URL}/{resource}", headers=headers, data=json.dumps(data))
    return response


# ==================================================================================================================== #

ORANGE_ID = 1
SPAGHETTI_ID = 2
APPLE_PIE_ID = 3


## Test 1: Post dishes
# Post three dishes { orange, spaghetti, and apple pie }
# (i) All three requests return unique ids
# (ii) Return 201 status code
def test_post_dishes():
    # Post orange
    data = {"name": "orange"}
    response = http_post("dishes", data)
    orange_id = response.text
    assert response.status_code == 201

    # Post spaghetti
    data = {"name": "spaghetti"}
    response = http_post("dishes", data)
    spaghetti_id = response.text
    assert response.status_code == 201

    # Post apple pie
    data = {"name": "apple pie"}
    response = http_post("dishes", data)
    apple_pie_id = response.text
    assert response.status_code == 201

    assert orange_id != spaghetti_id != apple_pie_id


# Test 2: Get dishes/<orange-id>
# Get sodium value of orange dish
# (i) Sodium field of JSON returns value between .9 and 1.1
# (ii) Return 200 status code
def test_get_dishes_name():
    response = http_get("dishes/" + str(ORANGE_ID))
    assert response.status_code == 200
    orange_data = response.json()
    assert .9 <= orange_data["sodium"] <= 1.1


# Test 3: Get dishes
# Get all dishes
# (i) JSON object has three embedded JSOn objects
# (ii) Return 200 status code
def test_get_dishes():
    response = http_get("dishes")
    assert response.status_code == 200
    assert len(response.json()) == 3


# Test 4: Post wrong dish
# Post a non existent dish
# (i) Return value is -3
# (ii) Return 400, 422 or 404 status code
def test_post_wrong_dish():
    data = {"name": "blah"}
    response = http_post("dishes", data)
    assert response.status_code == 400 or response.status_code == 422 or response.status_code == 404
    assert response.text == "-3"


# Test 5: Post already existing dish
# Post a dish that already exists { orange }
# (i) Return value is -2
# (ii) Return 400, 422 or 404 status code
def test_post_existing_dish():
    data = {"name": "orange"}
    response = http_post("dishes", data)
    assert response.status_code == 400 or response.status_code == 422 or response.status_code == 404
    assert response.text == "-2"


# Test 6: Post meal
# Post a meal { appetizer: orange, main: spaghetti, dessert: apple pie }
# (i) Return value > 0
# (ii) Return 201 status code
def test_post_meal():
    data = {"name": "delicious", "appetizer": ORANGE_ID, "main": SPAGHETTI_ID, "dessert": APPLE_PIE_ID}
    response = http_post("meals", data)
    assert response.status_code == 201
    assert int(response.text) > 0


# Test 7: Get meals
# Get all meals
# (i) JSON object has one embedded JSON object
# (ii) Meal has between 400 and 500 calories
# (iii) Return 200 status code
def test_get_meals():
    response = http_get("meals")
    assert response.status_code == 200
    assert len(response.json()) == 1
    meal = response.json()["1"]
    assert 400 <= meal["cal"] <= 500


# Test 8: Post existing meal
# Post a meal that already exists { appetizer: orange, main: spaghetti, dessert: apple pie }
# (i) Return value is -2
# (ii) Return 400 or 422 status code
def test_post_existing_meal():
    data = {"name": "delicious", "appetizer": ORANGE_ID, "main": SPAGHETTI_ID, "dessert": APPLE_PIE_ID}
    response = http_post("meals", data)
    assert response.status_code == 400 or response.status_code == 422
    assert response.text == "-2"
