from dataclasses import dataclass
from typing import Callable

@dataclass
class Option:
    name: str
    function: Callable

def record_blood_pressure():
    print('Entered the record_blood_pressure_menu')

def record_weight():
    print('Entered the record_weight menu')

def close_program():
    print('Goodbye!')

def string_can_be_integer(s):
    result = True
    try:
        i = int(s)
    except:
        result = False
    return result

def get_selection(input_text, num_options):
    while True:
        selection = input(input_text)
        if not string_can_be_integer(selection):
            print('Please make sure to input an integer.')
            continue
        selection = int(selection)
        options_range = range(1, num_options+1)
        if not selection in options_range:
            print(
                'Please make sure to input one of the following:',
                f'{", ".join(map(str, list(options_range)))}.'
            )
            continue
        return selection

def main():
    print('Welcome to BHealthy!')
    options = [
        Option('Record blood pressure', record_blood_pressure),
        Option('Record weight', record_weight),
        Option('Exit', close_program)
    ]
    print(f'Main menu:')
    num_options = len(options)
    for i, option in enumerate(options):
        print(f'{i+1}) {option.name}')
    task_selection = get_selection(
        'Which task would you like to complete? ', len(options)
    )
    options[task_selection-1].function()

if __name__ == '__main__':
    main()
