setenforce 0

DEST_IP=139.95.1.28
DEST_PORT=80
REDIRECT_IP=101.6.41.183
REDIRECT_PORT=21180

iptables -t nat -A OUTPUT -d $DEST_IP -p tcp --dport $DEST_PORT -j DNAT --to-destination $REDIRECT_IP:$REDIRECT_PORT
iptables -t nat -A OUTPUT -d $DEST_IP -p udp --dport $DEST_PORT -j DNAT --to-destination $REDIRECT_IP:$REDIRECT_PORT

iptables -t nat -A POSTROUTING -j MASQUERADE