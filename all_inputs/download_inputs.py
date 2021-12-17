import httpx
import os


if __name__ == "__main__":
    session_token = ""
    print("Starting")
    for i in range(25):
        path = f"aoc_{i}_input.txt"
        print(path)
        if not os.path.isfile(path):
            day_str = f"https://adventofcode.com/2021/day/{i}/input"
            response = httpx.get(day_str, cookies={"session": session_token})
            print(response)
            print(response.text)
            with open(path, "w") as file:
                file.write(response.text)

        