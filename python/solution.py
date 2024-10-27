def est_palindrome(mot):
    mot = mot.lower().replace(" ", "")
    return mot == mot[::-1]

# Exemple d'utilisation
mot1 = "radar"
mot2 = "Bonjour"
mot3 = "Eva, lève ce vélo !"

print(est_palindrome(mot1))  # True
print(est_palindrome(mot2))  # False
print(est_palindrome(mot3))  # True
n palindrome.")

if est_palindrome(mot2):
    print(f"{mot2} est un palindrome.")
else:
    print(f"{mot2} n'est pas un palindrome.")
