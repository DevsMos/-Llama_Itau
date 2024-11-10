# app.py
# app.py
@app.route('/')
def index():
    return render_template('index.html', accounts=accounts)

@app.route('/vote', methods=['POST'])
def vote():
    account = request.form['account']
    if account in accounts:
        accounts[account] += 1
        return jsonify({"message": f"Voto registrado para {account}!"})
    return jsonify({"message": "Conta inválida."}), 400


def process_payment(account):
    global fund_pool, donor_pool, dev_team_pool
    payment_amount = 100  # Quantia fictícia a ser paga

    if fund_pool >= payment_amount:
        # Calcular taxas
        fee = payment_amount * 0.05
        donation_share = fee * 0.6  # 3% para doadores
        dev_share = fee * 0.4       # 2% para a equipe

        # Distribuir taxas
        donor_pool += donation_share
        dev_team_pool += dev_share
        # Subtrair do fundo principal e enviar o restante para a conta
        fund_pool -= payment_amount
        actual_payment = payment_amount - fee

        return f"Pagamento de {actual_payment} unidades feito para {account}. Taxa: {fee}"
    else:
        return "Fundo insuficiente."

@app.route('/pay', methods=['POST'])
def pay():
    account = request.form['account']
    if accounts[account] > 0:
        result = process_payment(account)
        # Resetar votos após o pagamento
        accounts[account] = 0
        return jsonify({"message": result})
    else:
        return jsonify({"message": "Conta não possui votos suficientes para pagamento."}), 400
# app.py

def process_payment(account):
    global fund_pool, donor_pool, dev_team_pool
    payment_amount = 100  # Quantia fictícia a ser paga

    if fund_pool >= payment_amount:
        # Calcular taxas
        fee = payment_amount * 0.05
        donation_share = fee * 0.6  # 3% para doadores
        dev_share = fee * 0.4       # 2% para a equipe

        # Distribuir taxas
        donor_pool += donation_share
        dev_team_pool += dev_share

        # Subtrair do fundo principal e enviar o restante para a conta
        fund_pool -= payment_amount
        actual_payment = payment_amount - fee

        return f"Pagamento de {actual_payment} unidades feito para {account}. Taxa: {fee}"
    else:
        return "Fundo insuficiente."

@app.route('/pay', methods=['POST'])
def pay():
    account = request.form['account']
    if accounts[account] > 0:
        result = process_payment(account)
        # Resetar votos após o pagamento
        accounts[account] = 0
        return jsonify({"message": result})
    else:
        return jsonify({"message": "Conta não possui votos suficientes para pagamento."}), 400
