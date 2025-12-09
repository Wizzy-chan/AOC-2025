import requests
import os
from pathlib import Path

def get_input(day):
    input_path = (Path('.') / str(day) / 'input.txt')
    if not input_path.exists():
        print(f"Do not already have the input file for day {day}, fetching...")
        if 'SESSION_COOKIE' not in os.environ:
            print(f"Error fetching input for day {day}: SESSION_COOKIE environment variable not set.")
            print("Please set this to your AOC session cookie to fetch the input files")
            return False
            
        url = f'https://adventofcode.com/2025/day/{day}/input'
        cookies = { 'session': os.environ['SESSION_COOKIE'] }
        r = requests.get(url, cookies=cookies)
        if r.status_code != 200:
            print(f"Error fetching input for day {day}. HTTP status code: {r.status_code}")
            return False
        input_path.write_text(r.text)
    return True

def run_day(day):
    if get_input(day):
        print(f"Day {day}:")
        os.system(f'cd {day} && rustc -o main main.rs && ./main')
    
    
for i in range(1, 10):
    run_day(i)
