import { View, Text } from 'react-native'
import React, { useState } from 'react'
import { NativeStackNavigationProp, NativeStackScreenProps } from '@react-navigation/native-stack';
import { RootStackParamList } from '../../App';
import { Reactivo, Material, Auxiliar } from '../types/product';
import { useNavigation } from '@react-navigation/native';

type Props = NativeStackScreenProps<RootStackParamList, 'DetailsScreen'>;

export default function DetailScreen({ route }: Props) {



  const { product } = route.params;
  const [isEditing, setIsEditing] = useState(false);

  const handleEdit = () => {
    setIsEditing(true);
  };

  const renderProductDetails = () => {
    return Object.entries(product).map(([key, value]) => (
      <View key={key}>
        <Text>{key}</Text>
        <Text>{value}</Text>
      </View>
    ));
  };
  

  return (
    <View>
      {renderProductDetails()}
    </View>
  )
}