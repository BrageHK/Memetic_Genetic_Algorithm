import matplotlib.pyplot as plt

def read_data(file_name):
    with open(file_name, 'r') as file:
        lines = file.readlines()
    data = [list(map(int, line.strip().strip(',').split(','))) for line in lines]
    return data

def plot_data(data):
    names = ["max", "min", "mean"]
    for i, line in enumerate(data):
        plt.plot(range(len(line)), line, label=names[i])
    plt.xlabel('Index')
    plt.ylabel('Value')
    plt.title('Plot of Lines from File')
    plt.legend()
    plt.grid(True)
    plt.show()

# Main program
file_name = 'data/sga.txt'  # Replace with your file name
data = read_data(file_name)
plot_data(data)
