import sounddevice as sd

for i, device in enumerate(sd.query_devices()):
    print(
        f"{i}: {device['name']} ({'Input ✅' if device['max_input_channels'] > 0 else 'Output 🛑'})"
    )
