import requests
import json


class ConnectionController:

    URL = "http://127.0.0.1:8000"

    @staticmethod
    def http_get(resource: str):
        response = requests.get(url=f"{ConnectionController.URL}/{resource}", headers={"Content-Type": "application/json"})
        return response

    @staticmethod
    def http_delete(resource: str):
        response = requests.delete(url=f"{ConnectionController.URL}/{resource}", headers={"Content-Type": "application/json"})
        return response

    @staticmethod
    def http_post(resource: str, data: {}):
        response = requests.post(url=f"{ConnectionController.URL}/{resource}", headers={"Content-Type": "application/json"},
                                 data=json.dumps(data))
        return response

    @staticmethod
    def http_put(resource: str, data: {}):
        response = requests.put(url=f"{ConnectionController.URL}/{resource}", headers={"Content-Type": "application/json"},
                                data=json.dumps(data))
        return response

    @staticmethod
    def post_raw(resource: str, data: {}, headers: {}):
        response = requests.post(url=f"{ConnectionController.URL}/{resource}", headers=headers, data=json.dumps(data))
        return response

    @staticmethod
    def add_dish(name: str) -> int:
        dish = {"name": name}
        response = ConnectionController.http_post("dishes", dish)
        Assertion.assert_valid_added_resource(response)
        return response.json()

    @staticmethod
    def add_meal(name: str, appetizer_id: int, main_id: int, dessert_id: int) -> int:
        meal = {
            "name": name,
            "appetizer": appetizer_id,
            "main": main_id,
            "dessert": dessert_id
        }
        response = ConnectionController.http_post("meals", meal)
        Assertion.assert_valid_added_resource(response)
        assert response.json() > 0
        return response.json()


class Assertion:

    @staticmethod
    def assert_ret_value(response: requests.Response, returned_value: any):
        assert response.json() == returned_value

    @staticmethod
    def assert_err_code(response: requests.Response, error_code: int):
        assert response.status_code == error_code

    @staticmethod
    def assert_valid_added_resource(response: requests.Response):
        assert response.status_code == 201


orange_dish_id: int = None
spaghetti_dish_id: int = None
apple_pie_dish_id: int = None


def test_1():
    global orange_dish_id, apple_pie_dish_id, spaghetti_dish_id
    orange_dish_id = ConnectionController.add_dish("orange")
    spaghetti_dish_id = ConnectionController.add_dish("spaghetti")
    apple_pie_dish_id = ConnectionController.add_dish("apple pie")
    assert orange_dish_id != spaghetti_dish_id
    assert orange_dish_id != apple_pie_dish_id
    assert spaghetti_dish_id != apple_pie_dish_id


def test_2():
    global orange_dish_id
    assert orange_dish_id is not None

    response = ConnectionController.http_get(f"dishes/{orange_dish_id}")
    Assertion.assert_err_code(response, error_code=200)

    orange_sodium = response.json()["sodium"]
    assert 0.9 <= orange_sodium <= 1.1


def test_3():
    response = ConnectionController.http_get("dishes")
    Assertion.assert_err_code(response, error_code=200)

    dishes = response.json()
    assert len(dishes) == 3


def test_4():
    INVALID_DISH = {"name": "blah"}
    response = ConnectionController.http_post("dishes", INVALID_DISH)
    Assertion.assert_ret_value(response, -3)
    assert response.status_code == 404 or response.status_code == 400 or response.status_code == 422


def test_5():
    DISH_NAME = "orange"
    response = ConnectionController.http_post("dishes", {"name": DISH_NAME})
    Assertion.assert_ret_value(response, -2)
    assert response.status_code == 404 or response.status_code == 400 or response.status_code == 422


def test_6():
    global orange_dish_id, apple_pie_dish_id, spaghetti_dish_id
    assert orange_dish_id is not None
    assert apple_pie_dish_id is not None
    assert spaghetti_dish_id is not None

    ConnectionController.add_meal("delicious", orange_dish_id, spaghetti_dish_id, apple_pie_dish_id)


def test_7():
    response = ConnectionController.http_get("meals")
    Assertion.assert_err_code(response, error_code=200)
    meals = response.json()
    assert len(meals) == 1

    for key in meals:
        assert 400 <= meals[key]["cal"] <= 500


def test_8():
    global orange_dish_id, apple_pie_dish_id, spaghetti_dish_id
    assert orange_dish_id is not None
    assert apple_pie_dish_id is not None
    assert spaghetti_dish_id is not None
    meal = {"name": "delicious", "appetizer": orange_dish_id, "main": spaghetti_dish_id, "dessert": apple_pie_dish_id}
    response = ConnectionController.http_post("meals", meal)
    Assertion.assert_ret_value(response, returned_value=-2)
    assert response.status_code == 400 or response.status_code == 422
