import { View, Text, TouchableOpacity, StyleSheet, Image } from 'react-native';
import React from 'react';
import { createIconSet } from 'react-native-vector-icons';
const Feather = createIconSet(
  require('react-native-vector-icons/glyphmaps/Feather.json'),
  'Feather',
  'Feather.ttf'
);

interface EntityViewProps {
  imageUrl: string;
  title: string;
  onView?: () => void;
  onEdit?: () => void;
  onCreate?: () => void;
}

export default function EntityView({
  imageUrl,
  title,
  onView,
  onEdit,
  onCreate
}: EntityViewProps) {
  return (
    <View style={styles.container}>
      <Image
        source={{ uri: imageUrl }}
        style={styles.image}
        resizeMode="cover"
      />


      <View style={styles.footer}>
        <Text style={styles.title}>{title}</Text>

        <View style={styles.actionsContainer}>
          <TouchableOpacity
            style={styles.actionButton}
            onPress={onView}
          >
            <Feather name="eye" size={24} color="#007AFF" />
          </TouchableOpacity>

          <TouchableOpacity
            style={styles.actionButton}
            onPress={onEdit}
          >
            <Feather name="edit-2" size={24} color="#007AFF" />
          </TouchableOpacity>

          <TouchableOpacity
            style={styles.actionButton}
            onPress={onCreate}
          >
            <Feather name="plus-circle" size={24} color="#007AFF" />
          </TouchableOpacity>
        </View>

      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: '#f2f2f2',
    borderRadius: 15,
    overflow: 'hidden',
    marginBottom: 15,

  },
  image: {
    width: '100%',
    height: 200,
  },
  footer: {
    padding: 15,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#333',
    marginBottom: 10,
  },
  actionsContainer: {
    flexDirection: 'row',
    justifyContent: 'flex-end',
    alignItems: 'center',
  },
  actionButton: {
    padding: 8,
    marginLeft: 15,
  },
}); 