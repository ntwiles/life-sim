import sys
import re
import matplotlib.pyplot as plt
from collections import deque

def extract_info(log_line):
    pattern = r"Generation (\d+) over\. Survivors \d+/\d+ \(([\d\.]+)%\)"
    match = re.search(pattern, log_line)
    
    if match:
        generation_number = int(match.group(1))  # Group 1 captures the generation number
        survival_rate = float(match.group(2))    # Group 2 captures the survival rate percentage
        return generation_number, survival_rate
    else:
        raise ValueError("String does not match the expected pattern")


def main():
    display_window_size = 500
    trend_window_size = 1000
    running_sum = 0.0

    plt.ion()
    fig, ax = plt.subplots()
    fig.patch.set_facecolor('#12161e')
    ax.set_facecolor('#12161e')

    for spine in ax.spines.values():
            spine.set_edgecolor('#cccccc')

    x_data = deque(maxlen=display_window_size)
    y_data = deque(maxlen=display_window_size)
    y_trend = deque(maxlen=display_window_size)
    trend_data = deque(maxlen=trend_window_size)

    for line in sys.stdin:
        print(line)
        generation_number, survival_rate = extract_info(line)

        if len(trend_data) >= trend_window_size:
            running_sum -= trend_data[0]

        trend_data.append(survival_rate)
        running_sum += survival_rate

        trend = running_sum / len(trend_data)

        x_data.append(generation_number)
        y_data.append(survival_rate)
        y_trend.append(trend)
        
        ax.clear()
        ax.plot(x_data, y_data)
        ax.plot(x_data, y_trend)
        ax.set_xlabel("Generation", color='#cccccc')
        ax.set_ylabel("Survival Rate (%)", color='#cccccc')
        ax.tick_params(axis='x', colors='#cccccc')
        ax.tick_params(axis='y', colors='#cccccc')
        
        ax.legend()
        
        plt.draw()
        plt.pause(1)

if __name__ == '__main__':
    main()