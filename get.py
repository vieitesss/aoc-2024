#!./.venv/bin/python3

import requests
import argparse
import os


file_path = "session.id"
input_directory = "./input/"


def get_session_id():
    try:
        with open(file_path, "r") as file:
            session_id = file.readline().strip()
            return session_id
    except FileNotFoundError:
        print(f"Error: El archivo '{file_path}' no existe.")
        return None
    except Exception as e:
        print(f"Error al leer el archivo '{file_path}': {e}")
        return None


def download_file_with_sessionid(url, sessionid, output_file):
    cookies = {"session": sessionid}

    try:
        response = requests.get(url, cookies=cookies, stream=True)
        response.raise_for_status()

        with open(output_file, "wb") as file:
            for chunk in response.iter_content(chunk_size=8192):
                file.write(chunk)

        print(f"Archivo descargado exitosamente como: {output_file}")

    except requests.exceptions.RequestException as e:
        print(f"Error al descargar el archivo: {e}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Número del día a descargar")
    parser.add_argument(
        "number", type=int, help="Un número entre 1 y 25 (ambos incluidos)."
    )
    args = parser.parse_args()

    # Validar el rango del número
    if not (1 <= args.number <= 25):
        print("Error: El número debe estar entre 1 y 25 (ambos incluidos).")
        exit(1)

    url = f"https://adventofcode.com/2024/day/{args.number}/input"
    sessionid = get_session_id()
    output_file = f"day{args.number}"
    final_path = os.path.join(input_directory, output_file)

    download_file_with_sessionid(url, sessionid, final_path)
