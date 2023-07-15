import requests
from bs4 import BeautifulSoup
import json

# Scrape the website
url = 'https://docs.python.org/3/tutorial/appetite.html'  # Replace with the website URL you want to scrape
response = requests.get(url)
soup = BeautifulSoup(response.content, 'html.parser')

# Define the data structure
data = []

# Find headers and paragraphs
headers = soup.find_all('h1')
for header in headers:
    header_text = header.text.strip()
    paragraphs = header.find_next_siblings('p')
    paragraphs_text = [p.text.strip() for p in paragraphs]
    data.append({
        'header': header_text,
        'paragraphs': paragraphs_text
    })

# Save data to a file
output_file = '../frontend/data/scraped_data.json'
with open(output_file, 'w') as file:
    json.dump(data, file, indent=4)

print(f"Scraped data saved to {output_file}")
