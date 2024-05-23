import { Day, getISODateString } from 'lembas-api';
import { ReactNode, memo, useEffect } from 'react';
import { TouchableOpacity, View } from 'react-native';
import { Icon, IconButton, Surface, Text, useTheme } from 'react-native-paper';
import Animated, { useAnimatedStyle, useSharedValue, withSpring } from 'react-native-reanimated';

// JS has day 0 as Sunday
const DAYS = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];

interface DayButtonProps {
	isToday: boolean;
	date: Date;
	onPress: () => void;
	decoration: ReactNode;
}
function DayButton(props: DayButtonProps) {
	const theme = useTheme();

	const { isToday, date, onPress, decoration } = props;

	const color = isToday ? theme.colors.tertiaryContainer : theme.colors.primaryContainer;
	return (
		<Surface
			style={{
				flex: 1,
				backgroundColor: color,
				borderRadius: theme.roundness,
				overflow: 'hidden'

			}}>
			<TouchableOpacity
				style={{
					padding: 6,
					flexDirection: 'column',
					alignItems: 'center',

				}}
				onPress={onPress}
			>
				<Text
					variant='titleSmall'
					style={{
						color: theme.colors.onSecondaryContainer,
						letterSpacing: 1,
						fontWeight: isToday ? 'bold' : 'normal'
					}}>
					{shortDay(date.getDay())}
				</Text>
				<Text
					variant='bodyMedium'
					style={{
						color: theme.colors.tertiary
					}}>
					{date.getDate()}
				</Text>
				{decoration}
			</TouchableOpacity >
		</Surface >);
}
export interface DaySelectorProps {
	/**
	 * The first date to render.
	 */
	firstDay: Date;
	/**
	 * The current meal plan (to highlight days which have recipes selected)
	 */
	planDays: Day[];
	/**
	 * Called when a day is pressed. 
	 */
	onPressDay: (day: Date) => void;
	/**
	 * Called when the left and right arrows are pressed.
	 */
	onPressLeftArrow: () => void;
	onPressRightArrow: () => void;
	/**
	 * Whether the reset button is enabled, and what to do when it's pressed
	 */
	resetEnabled: boolean;
	onPressReset: () => void;
}

export function DaySelector(props: DaySelectorProps): JSX.Element {
	const { firstDay, planDays, onPressDay, onPressLeftArrow, onPressRightArrow, resetEnabled, onPressReset: onPressClock } = props;
	const today = new Date(Date.now());

	const days = getDateRange(firstDay, 7).map((date, i) => {
		const day = planDays.find(d => d.date === getISODateString(date));
		const isToday = getISODateString(today) === getISODateString(date);
		const nIndicators = day ? day.recipes.length : 0;

		const MemoizedIndicators = memo((props: AnimatedIndicatorsProps) => <AnimatedIndicators n={props.n} />);
		return <DayButton
			key={i}
			isToday={isToday}
			date={date}
			onPress={() => onPressDay(date)}
			decoration={
				<MemoizedIndicators key={i} n={nIndicators} />
			} />;
	});


	return (
		<View style={{ padding: 5 }}>
			<View style={{ flexDirection: 'row', gap: 6, marginTop: 10 }}>
				{days}

			</View>
			<View style={{ flexDirection: 'row', gap: 50, marginTop: 10, justifyContent: 'center' }}>
				<IconButton icon={'chevron-left'} onPress={() => onPressLeftArrow()} />
				<IconButton disabled={!resetEnabled} icon={'calendar-today'} onPress={() => onPressClock()} />
				<IconButton icon={'chevron-right'} onPress={() => onPressRightArrow()} />

			</View>
		</View>
	);
}

/**
 * Renders a sequence of `n` indicator dots.
 */
interface AnimatedIndicatorsProps {
	n: number;
}

function AnimatedIndicators(props: AnimatedIndicatorsProps): JSX.Element {
	const { n } = props;
	const theme = useTheme();
	const translateY = useSharedValue(15);
	translateY.value = 0;

	const animatedStyle = useAnimatedStyle(() => ({
		width: '60%',
		transform: [{ translateY: withSpring(translateY.value, { duration: 400 }) }],
	}));

	useEffect(() => {
	}, []);

	let indicators: JSX.Element[];

	if (n === 0) {
		// Render a placeholder component if no indicators are to be rendered.
		indicators = [<View key={1} style={{ height: 5 }} />];
	}
	else {
		indicators = [...Array(n).keys()].map((v, i) => <Icon key={i} color={theme.colors.primary} source={'circle'} size={5} />);
	}

	return (
		<Animated.View style={animatedStyle}>
			<View
				style={{
					marginTop: 5,
					justifyContent: 'space-around',
					flexDirection: 'row',
				}}>
				{indicators}
			</View>
		</Animated.View>
	);
}

function shortDay(day: number) {
	return DAYS[day].substring(0, 3).toUpperCase();
}

/**
 * Generates a range of dates.
 */
function getDateRange(from: Date, range: number): Date[] {
	const start = new Date(from);
	const dates = [];
	if (range > 0) {
		for (const i of Array(range).keys()) {
			const date = new Date(start);
			date.setDate(start.getDate() + i);
			dates.push(date);
		}
	}
	return dates;
}
