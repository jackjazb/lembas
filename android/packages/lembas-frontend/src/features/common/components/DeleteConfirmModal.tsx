import { Button, Card, Modal, Text } from 'react-native-paper';

export interface DeleteConfirmModalProps {
	title: string;
	open: boolean;
	onConfirm: () => void;
	onDismiss: () => void;
}
export function DeleteConfirmModal(props: DeleteConfirmModalProps) {
	const { title, open, onConfirm, onDismiss } = props;

	return (
		<Modal visible={open} onDismiss={() => onDismiss()} contentContainerStyle={{ padding: 20 }}>
			<Card mode='contained' >

				<Card.Content style={{ gap: 10 }}>
					<Text variant='titleLarge'>
						{title}
					</Text>
					<Text variant='bodyLarge'>
						This cannot be undone.
					</Text>
				</Card.Content>

				<Card.Actions style={{ marginTop: 10 }}>
					<Button onPress={() => onDismiss()}>
						Cancel
					</Button>
					<Button onPress={() => {
						onConfirm();
					}}>
						Confirm
					</Button>
				</Card.Actions>
			</Card>
		</Modal >
	);
}